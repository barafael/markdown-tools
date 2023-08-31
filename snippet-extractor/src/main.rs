use std::{fs::read_to_string, path::PathBuf};

use crate::parser::{Rule, SnippetParser};
use clap::Parser as ClapParser;
use pest::Parser;
use snippet::{RelativePathBuf, Snippet, Snippets};
use walkdir::DirEntry;

mod parser;

#[derive(Debug, ClapParser)]
#[command(author, version)]
pub struct Arguments {
    #[arg(short, long)]
    directory: PathBuf,

    #[arg(short, long, default_value_t = false)]
    relative: bool,

    #[arg(short, long, default_value = "snippets.json")]
    output: PathBuf,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let mut map = Snippets::new();

    for entry in walkdir::WalkDir::new(args.directory)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
    {
        if entry.path().is_file() {
            let content = read_to_string(entry.path())?;

            let pairs = SnippetParser::parse(Rule::File, &content)?;
            for pair in pairs.into_iter().next().unwrap().into_inner() {
                if pair.as_rule() == Rule::Snippet {
                    let (line, col) = pair.line_col();
                    let mut snippet = pair.into_inner();
                    let identifier = snippet.next().unwrap().as_str().to_string();
                    let snippet_text = snippet.next().unwrap().as_str().to_string();
                    let snippet = Snippet {
                        content: snippet_text,
                        file: RelativePathBuf::from_path(entry.path()).unwrap(),
                        line,
                        col,
                    };
                    map.insert(identifier, snippet);
                }
            }
        }
    }
    let json = serde_json::to_string_pretty(&map)?;
    std::fs::write(args.output, json)?;
    Ok(())
}
