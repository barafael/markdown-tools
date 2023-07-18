use clap::Parser as ClapParser;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use std::path::PathBuf;
use std::{fs, io::Write};

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    #[arg(short, long, default_value = "snippets.json")]
    snippets: PathBuf,

    #[arg(short, long)]
    markdown_file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let input = fs::read_to_string(args.markdown_file).unwrap();

    let parser = Parser::new(&input);

    let mut in_code_block = false;
    let mut current_context = None;
    let mut current_block = None;

    let i = parser.collect::<Vec<_>>();
    let mut new_vec = Vec::with_capacity(i.len());
    for event in i {
        match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                if let CodeBlockKind::Fenced(text) = kind {
                    let context = text
                        .split(", ")
                        .filter(|s| s.starts_with("context:"))
                        .collect::<Vec<_>>()
                        .pop();
                    if let Some(context) = context {
                        let context = context.split_once(':').unwrap().1.to_string();
                        current_context = Some(context);
                    }
                }
                in_code_block = true;
                new_vec.push(event);
            }
            Event::Text(text) if in_code_block => {
                current_block = Some(text.to_string());
                new_vec.push(Event::Text(text));
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
                new_vec.push(event);
                let context = current_context.take();
                let _block = current_block.take();
                let tag = format!(
                    r#"<button name="button" onclick="console.log('{}')">Click me</button>"#,
                    context.unwrap_or_default()
                );

                let new_event = Event::Html(pulldown_cmark::CowStr::Boxed(tag.into()));
                new_vec.push(new_event);
            }
            _ => new_vec.push(event),
        };
    }
    let mut buf = String::with_capacity(input.len() + 1000);

    let _state = cmark(&mut new_vec.into_iter(), &mut buf)?;

    let mut stdout = std::io::stdout();

    stdout.write_all(buf.as_bytes())?;
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
