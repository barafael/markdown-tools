use std::{fs::read_to_string, io::Write, path::PathBuf};

use anyhow::Context;
use clap::Parser as ClapParser;
use ignore::Walk;
use parser::parse;
use path_dedot::ParseDot;
use snippet_extractor::Snippets;

pub(crate) mod parser;

#[derive(Debug, ClapParser)]
#[command(author, version)]
pub struct Arguments {
    #[arg(short, long)]
    directory: PathBuf,

    #[arg(short, long, default_value_t = false)]
    relative: bool,

    #[arg(short, long, default_value = "snippets.json")]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let mut map = Snippets::default();

    let directory = args
        .directory
        .parse_dot()
        .context("Failed to parse input directory")?;
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    for entry in Walk::new(directory).filter_map(Result::ok) {
        if entry.path().is_file() {
            let content = read_to_string(entry.path())?;
            let path = if args.relative {
                entry
                    .path()
                    .strip_prefix(&current_dir)
                    .context("Could not create relative path")?
            } else {
                entry.path()
            };
            let snippets = parse(&content, path);
            if !snippets.is_empty() {
                map.snippets.insert(path.to_path_buf(), snippets);
            }
        }
    }

    let json = serde_json::to_string_pretty(&map)?;

    if let Some(path) = args.output {
        std::fs::write(path, json)?;
    } else {
        let mut stdout = std::io::stdout();
        stdout.write_all(json.as_bytes())?;
    }

    Ok(())
}
