use std::{
    io::Write,
    process::{Command, Stdio},
};

use itertools::Itertools;
use pulldown_cmark::CowStr;
use serde::{Deserialize, Serialize};
use take_until::TakeUntilExt;
use urlencoding::encode;

use super::ButtonInserter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Template {
    pre: String,
    post: String,
}

#[derive(Debug, Default)]
pub struct PlaygroundButtonInserter;

impl ButtonInserter for PlaygroundButtonInserter {
    fn handle_codeblock(
        &self,
        fence: &CowStr,
        code: &CowStr,
        _current_block: &mut Option<String>,
        current_url: &mut Option<String>,
        current_btn_text: &mut Option<String>,
    ) {
        let main_template = Template {
            pre: String::from("fn main() {"),
            post: String::from("}"),
        };
        let main_anyhow_template = Template {
            pre: String::from("fn main() -> anyhow::Result<()> {"),
            post: String::from("Ok(())\n}"),
        };
        let tokio_main_anyhow_template = Template {
            pre: String::from("#[tokio::main]\nasync fn main() -> anyhow::Result<()> {"),
            post: String::from("Ok(())\n}"),
        };

        let tag = fence
            .split_whitespace()
            .filter(|s| s.starts_with("tag:"))
            .collect::<Vec<_>>()
            .pop();
        if tag != Some("tag:playground-button") {
            return;
        }

        let template = if fence.split_whitespace().contains(&"playground-wrap:main") {
            main_template
        } else if fence
            .split_whitespace()
            .contains(&"playground-wrap:main_anyhow")
        {
            main_anyhow_template
        } else if fence
            .split_whitespace()
            .contains(&"playground-wrap:main_tokio_anyhow")
        {
            tokio_main_anyhow_template
        } else {
            if fence.split_whitespace().contains(&"playground-wrap:") {
                eprintln!("Warning: unknown playground wrap marker in \"{fence}\"");
            }
            let before = fence
                .split_whitespace()
                .skip_while(|elem| !elem.starts_with("playground-before:$\""))
                .take_until(|elem| elem.ends_with("\"$"))
                .join(" ");
            let after = fence
                .split_whitespace()
                .skip_while(|elem| !elem.starts_with("playground-after:"))
                .take_until(|elem| elem.ends_with("$\""))
                .join(" ");
            let before = before.replace("playground-before:$\"", "");
            let before = before.replace("\"$", "");
            let after = after.replace("playground-after:$\"", "");
            let after = after.replace("\"$", "");

            Template {
                pre: before,
                post: after,
            }
        };

        let before = template.pre;
        let after = template.post;
        let text = format!("{before}{code}{after}");
        let text = text.replace("\\n", "\n");

        let text = format_rust_code_block(&text);

        let text = encode(&text);
        let text = format!(
            "'https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code={text}'"
        );

        *current_url = Some(text);
        *current_btn_text = Some("Playground".into());
    }
}

fn format_rust_code_block(code: &str) -> String {
    // Run the rustfmt command line tool on the code block
    let output = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("--edition=2021")
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .as_mut()
                .expect("Failed to get stdin of child process")
                .write_all(code.as_bytes())?;
            let output = child.wait_with_output()?;
            Ok(output.stdout)
        });

    // Return the formatted code as a String, or the original code block if there was an error
    match output {
        Ok(vec) if vec.is_empty() => {
            eprintln!("Failed to format Rust code.");
            code.to_owned()
        }
        Ok(formatted_output) => String::from_utf8_lossy(&formatted_output).into_owned(),
        Err(e) => {
            eprintln!("Failed to format Rust code: {e:?}");
            code.to_owned()
        }
    }
}
