use clap::Parser as ClapParser;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use snippet::Snippets;
use std::path::PathBuf;
use std::{fs, io::Write};

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    #[arg()]
    markdown_file: PathBuf,

    #[arg(long, default_value_t = true)]
    button: bool,

    #[arg(short, long, default_value = "snippets.json")]
    snippets: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let snippets = fs::read_to_string(args.snippets).unwrap();
    let snippets: Snippets = serde_json::from_str(&snippets)?;

    let input = fs::read_to_string(args.markdown_file).unwrap();

    let parser = Parser::new(&input);

    let mut current_url = None;
    let mut current_block = None;

    let i = parser.collect::<Vec<_>>();
    let mut new_vec = Vec::with_capacity(i.len());
    for event in i {
        match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                if let CodeBlockKind::Fenced(text) = kind {
                    let context = text
                        .split_whitespace()
                        .filter(|s| s.starts_with("marker:"))
                        .collect::<Vec<_>>()
                        .pop();

                    if let Some(marker) = context {
                        let marker = marker.split_once(':').unwrap().1;
                        if let Some(value) = snippets.get(marker) {
                            let content = &value.content;
                            let dedented = textwrap::dedent(content);
                            current_block = Some(dedented);

                            let url = if value.file.is_absolute() {
                                format!(
                                    "    onclick=\"window.location.href='vscode://file{}:{}:{}'\"\n",
                                    value.file.display(),
                                    value.line + 1,
                                    value.col
                                )
                            } else {
                                todo!("handle relative paths using window.location.href");
                            };
                            current_url = Some(url);
                        }
                    }
                }
                new_vec.push(Event::Html("<div style=\"position: relative;\">".into()));
                new_vec.push(event);
            }
            Event::Text(text) => {
                let event = Event::Text(current_block.take().map(CowStr::from).unwrap_or(text));
                new_vec.push(event);
            }
            Event::End(Tag::CodeBlock(_)) => {
                new_vec.push(event);
                if args.button {
                    new_vec.push(Event::Html("<p style=\"position: absolute; right: 10px; top: 10px; padding: 0; margin: 0; line-height: 0\">\n".into()));
                    new_vec.push(Event::Html("<button\n".into()));
                    new_vec.push(Event::Html(current_url.take().unwrap_or_default().into()));
                    new_vec.push(Event::Html("    style=\"\n".into()));
                    new_vec.push(Event::Html("    height: fit-content;\n".into()));
                    new_vec.push(Event::Html("    margin: 0;\n".into()));
                    new_vec.push(Event::Html("    font-weight: bold;\"\n".into()));
                    new_vec.push(Event::Html(">OPEN VSCODE\n".into()));
                    new_vec.push(Event::Html("</button>\n".into()));
                    new_vec.push(Event::Html("</p>\n".into()));
                    new_vec.push(Event::Html("</div>\n".into()));
                }
            }
            _ => new_vec.push(event),
        };
    }
    let mut buf = String::with_capacity(input.len() + 1000);

    let _state = cmark(&mut new_vec.into_iter(), &mut buf)?;

    if let Some(path) = args.output {
        std::fs::write(path, buf)?;
    } else {
        let mut stdout = std::io::stdout();
        stdout.write_all(buf.as_bytes())?;
    }

    Ok(())
}
