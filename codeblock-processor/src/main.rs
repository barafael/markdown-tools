use anyhow::Context;
use clap::Parser as ClapParser;
use processor::playground_button_inserter::PlaygroundButtonInserter;
use processor::snippet_button_inserter::SnippetButtonInserter;
use processor::ButtonInserter;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use snippet_extractor::Snippets;
use std::path::PathBuf;
use std::{fs, io::Write};

mod processor;

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    #[arg()]
    input: PathBuf,

    #[arg(long, default_value_t = true)]
    button: bool,

    #[arg(short, long)]
    snippets: Option<PathBuf>,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let snippets = if let Some(snippets) = args.snippets {
        let snippets = fs::read_to_string(snippets).context("Failed to load snippets")?;
        serde_json::from_str::<Snippets>(&snippets).context("Failed to parse snippets")?
    } else {
        Snippets::default()
    };

    let input = fs::read_to_string(args.input).context("Failed to open input file")?;

    let parser = Parser::new(&input);

    let snippet_inserter = SnippetButtonInserter::with_snippets(snippets);
    let playground_inserter = PlaygroundButtonInserter;

    let mut current_url = None;
    let mut current_block = None;
    let mut current_fence = None;
    let mut current_btn_text = None;

    let i = parser.collect::<Vec<_>>();
    let mut document = Vec::with_capacity(i.len());

    for event in i {
        match event.clone() {
            Event::Start(Tag::CodeBlock(kind)) => {
                if let CodeBlockKind::Fenced(text) = kind {
                    current_fence = Some(text);
                }
                if args.button {
                    document.push(Event::Html("<div style=\"position: relative;\">".into()));
                }
                document.push(event);
            }
            Event::Text(ref code) => {
                if let Some(ref fence) = current_fence.take() {
                    snippet_inserter.handle_codeblock(
                        fence,
                        code,
                        &mut current_block,
                        &mut current_url,
                        &mut current_btn_text,
                    );
                    playground_inserter.handle_codeblock(
                        fence,
                        code,
                        &mut current_block,
                        &mut current_url,
                        &mut current_btn_text,
                    );
                }

                let event = Event::Text(
                    current_block
                        .take()
                        .map_or_else(|| code.clone(), CowStr::from),
                );
                document.push(event);
            }
            Event::End(Tag::CodeBlock(_)) => {
                document.push(event);
                if args.button {
                    if let Some(url) = current_url.take() {
                        let btn_text = current_btn_text.take().unwrap_or_default();
                        let button = make_button(url, btn_text);
                        document.extend(button.into_iter());
                    }
                    document.push(Event::Html("</div>\n\n".into()));
                }
            }
            _ => document.push(event),
        };
    }

    document.push(Event::Html(include_str!("make_path.html").into()));
    document.push(Event::Text("\n".into()));

    document.push(Event::Html(include_str!("blur_button.html").into()));
    document.push(Event::Text("\n".into()));

    let mut output = String::with_capacity(input.len() + 1000);

    let _state = cmark(&mut document.into_iter(), &mut output)?;

    if let Some(path) = args.output {
        std::fs::write(path, output)?;
    } else {
        let mut stdout = std::io::stdout();
        stdout.write_all(output.as_bytes())?;
    }

    Ok(())
}

fn make_button<'a>(url: String, btn_text: String) -> Vec<Event<'a>> {
    let mut button = Vec::new();
    button.push(Event::Html("<p style=\"position: absolute; right: 10px; top: 10px; padding: 0; margin: 0; line-height: 0\">\n".into()));
    button.push(Event::Html("<button\n    onclick=\"window.open(".into()));
    button.push(Event::Html(url.into()));
    button.push(Event::Html(",'_blank')\"\n".into()));
    button.push(Event::Html("    style=\"\n".into()));
    button.push(Event::Html("    height: fit-content;\n".into()));
    button.push(Event::Html("    margin: 0;\n".into()));
    button.push(Event::Html("    font-weight: bold;\"\n".into()));
    button.push(Event::Html(format!(">{}\n", btn_text).into()));
    button.push(Event::Html("</button>\n".into()));
    button.push(Event::Html("</p>\n".into()));
    button
}

#[cfg(test)]
mod test {
    use pulldown_cmark_to_cmark::cmark;

    use crate::make_button;

    #[test]
    fn makes_button() {
        let mut output = String::new();
        let button = make_button(
            "'https://www.example.com'".to_string(),
            "Example.com!".into(),
        );
        let _state = cmark(&mut button.into_iter(), &mut output).unwrap();

        let expected = r#"<p style="position: absolute; right: 10px; top: 10px; padding: 0; margin: 0; line-height: 0">
<button
    onclick="window.open('https://www.example.com','_blank')"
    style="
    height: fit-content;
    margin: 0;
    font-weight: bold;"
>Example.com!
</button>
</p>
"#;
        assert_eq!(expected, output);
    }
}
