use std::{collections::HashMap, path::PathBuf};

use crate::parser::{Rule, SnippetParser};
use clap::Parser as ClapParser;
use pest::Parser;
use walkdir::DirEntry;

mod parser;

#[derive(Debug, ClapParser)]
#[command(author, version)]
pub struct Arguments {
    #[arg(short, long)]
    directory: PathBuf,

    #[arg(short, long, default_value = "snippets.json")]
    output: PathBuf,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let mut map = HashMap::new();

    for entry in walkdir::WalkDir::new(args.directory)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            let content = std::fs::read_to_string(entry.path())?;

            let pairs = SnippetParser::parse(Rule::file, &content)?;
            for pair in pairs.into_iter().next().unwrap().into_inner() {
                if pair.as_rule() == Rule::snippet {
                    let mut snippet = pair.into_inner();
                    let identifier = snippet.next().unwrap().as_str().to_string();
                    let snippet_text = snippet.next().unwrap().as_str().to_string();
                    map.insert(identifier, snippet_text);
                }
            }
        }
    }
    let json = serde_json::to_string_pretty(&map)?;
    std::fs::write(args.output, json)?;
    Ok(())
}
