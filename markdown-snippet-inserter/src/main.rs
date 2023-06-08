use clap::Parser as ClapParser;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io::Write};

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    #[arg(short, long, default_value = "snippets.json")]
    snippets: PathBuf,

    #[arg(short, long)]
    markdown_file: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let snippets = fs::read_to_string(args.snippets).unwrap();
    let snippets: HashMap<String, String> = serde_json::from_str(&snippets)?;

    let input = fs::read_to_string(args.markdown_file).unwrap();

    let parser = Parser::new(&input);

    let mut in_code_block = false;
    let mut current_snippet = None;

    let mut i = parser.map(|event| match event {
        Event::Start(Tag::CodeBlock(ref kind)) => {
            println!("start code block: {kind:?}");
            if let CodeBlockKind::Fenced(text) = kind {
                let marker = text
                    .split(" ")
                    .filter(|s| s.starts_with("marker:"))
                    .collect::<Vec<_>>()
                    .pop();
                if let Some(marker) = marker {
                    let marker = marker.split_once(':').unwrap().1;
                    if let Some(value) = snippets.get(marker) {
                        dbg!(&value);
                        let dedented = textwrap::dedent(value);
                        current_snippet = Some(dedented);
                    }
                }
            }
            in_code_block = true;
            event
        }
        Event::Text(text) if in_code_block => {
            println!("Text event");
            println!("{current_snippet:?}");
            if let Some(value) = current_snippet.take() {
                println!("Replacing {text} with {value}");
                Event::Text(pulldown_cmark::CowStr::Boxed(value.into()))
            } else {
                Event::Text(text)
            }
        }
        Event::End(Tag::CodeBlock(_)) => {
            println!("Ending codeblock.");
            in_code_block = false;
            current_snippet = None;
            event
        }
        _ => event,
    });

    let mut buf = String::with_capacity(input.len());
    let _state = cmark(&mut i, &mut buf)?;

    if let Some(path) = args.output {
        std::fs::write(path, buf)?;
    } else {
        let mut stdout = std::io::stdout();
        stdout.write_all(buf.as_bytes())?;
    }

    Ok(())
}

/*
fn format_rust_code_block_crate(code: &str) -> String {
    // Set up Rustfmt configuration with default options
    let mut config = Config::default();
    config.set().emit_mode(EmitMode::Stdout);

    // Run Rustfmt on the code block
    let input = Input::Text(code.to_owned());
    let mut output = Vec::new();
    if let Err(e) = rustfmt::format_input(input, &config, Some(&mut output)) {
        eprintln!("Failed to format Rust code: {:?}", e);
        return code.to_owned();
    }

    // Return the formatted code as a String
    String::from_utf8_lossy(&output).into_owned()
}
*/

/*
fn format_rust_code_block(code: &str) -> String {
    // Run the rustfmt command line tool on the code block
    let output = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("--edition=2021")
        .spawn()
        .and_then(|mut child| {
            child.stdin.as_mut().unwrap().write_all(code.as_bytes())?;
            let output = child.wait_with_output()?;
            Ok(output.stdout)
        });

    // Return the formatted code as a String, or the original code block if there was an error
    match output {
        Ok(formatted_output) => String::from_utf8_lossy(&formatted_output).into_owned(),
        Err(e) => {
            eprintln!("Failed to format Rust code: {e:?}");
            code.to_owned()
        }
    }
}
*/
