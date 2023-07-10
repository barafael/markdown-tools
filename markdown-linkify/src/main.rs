use anyhow::Context;
use clap::Parser as ClapParser;
use markdown_linkify::docs_rustlang_replacer::DocsRustlangReplacer;
use markdown_linkify::docsrs_replacer::DocsrsReplacer;
use markdown_linkify::regex::RegexReplacer;
use markdown_linkify::{linkify, Replacer};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io::Write};

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    /// The input markdown file.
    input: PathBuf,

    /// Configuration file in TOML format.
    #[arg(short, long, default_value = "linkify.toml")]
    config: PathBuf,

    /// The output file, or stdout if not specified.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Replacers {
    regexes: Vec<RegexReplacer>,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    /*
    let example: Replacers = Replacers {
        regexes: vec![RegexReplacer::example(), RegexReplacer::example()],
    };
    println!("{}", toml::to_string_pretty(&example).unwrap());
    */

    let regex_replacers = fs::read_to_string(&args.config)
        .with_context(|| format!("Failed to read config file at {:?}", args.config))?;
    let regex_replacers: Replacers = toml::from_str(&regex_replacers)?;

    let mut replacers: Vec<Box<dyn Replacer>> = Vec::new();

    for replacer in regex_replacers.regexes {
        replacers.push(Box::new(replacer));
    }
    replacers.push(Box::new(DocsRustlangReplacer::new()));
    replacers.push(Box::new(DocsrsReplacer::new()));

    let input = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {:?}", args.input))?;

    let buf = linkify(&input, &replacers)?;
    if let Some(path) = &args.output {
        std::fs::write(path, buf)
            .with_context(|| format!("Failed to write output file: {:?}", args.output))?;
    } else {
        let mut stdout = std::io::stdout();
        stdout
            .write_all(buf.as_bytes())
            .context("Failed to write to stdout")?;
    }

    Ok(())
}
