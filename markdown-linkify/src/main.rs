use clap::Parser as ClapParser;
use markdown_linkify::config::Config;
use markdown_linkify::docs_rustlang_replacer::DocsRustlangReplacer;
use markdown_linkify::docsrs_replacer::DocsrsReplacer;
use markdown_linkify::linkify;
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

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    /*
    let config = Config {
        replacements: vec![Replacement::Regex {
            pattern: Regex::new(r"SP-(?<s>\d+)").unwrap(),
            replacement: "jira.com/SP-$s".to_string(),
            limit: 3,
        }],
    };
    println!("{}", toml::to_string_pretty(&config).unwrap());
    */
    let config = fs::read_to_string(&args.config)?;
    let mut config: Config = toml::from_str(&config)?;

    config.register_callback(Box::new(DocsRustlangReplacer::new()));
    config.register_callback(Box::new(DocsrsReplacer::new()));

    let input = fs::read_to_string(args.input).unwrap();

    let buf = linkify(&input, &config)?;
    if let Some(path) = args.output {
        std::fs::write(path, buf)?;
    } else {
        let mut stdout = std::io::stdout();
        stdout.write_all(buf.as_bytes())?;
    }

    Ok(())
}
