use anyhow::Context;
use clap::Parser as ClapParser;
use markdown_linkify::docs_rustlang_replacer::DocsRustlang;
use markdown_linkify::docsrs_replacer::Docsrs;
use markdown_linkify::{
    broken_link_callback_with_replacers, process_broken_links, process_links, LinkTransformer,
    Transformers,
};
use pulldown_cmark_to_cmark::cmark;
use std::path::PathBuf;

use std::{fs, io::Write};

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    /// The input markdown file.
    input: PathBuf,

    /// Configuration file in TOML format.
    #[arg(short, long, default_value = "linkify.toml")]
    config: PathBuf,

    /// Write example configuration file?
    /// No further action is taken.
    #[arg(short, long)]
    example: bool,

    /// The output file, or stdout if not specified.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    if args.example {
        let example = Transformers::example();
        let example = toml::to_string_pretty(&example)?;
        std::fs::write(&args.config, example)?;
        return Ok(());
    }

    let regex_replacers: Transformers = toml::from_str(
        fs::read_to_string(&args.config)
            .context("Failed to read transformer config file")?
            .as_str(),
    )
    .context("Failed to deserialize toml config file")?;

    // Create a vector with `LinkTransformer` trait objects.
    // Some of which are read in from the config file.
    // Others are added hard-coded, such as the rustdoc and docs.rs replacers.
    let mut replacers: Vec<Box<dyn LinkTransformer>> = Vec::new();
    for replacer in regex_replacers.regex {
        replacers.push(Box::new(replacer));
    }
    replacers.push(Box::new(DocsRustlang::new()));
    replacers.push(Box::new(Docsrs::new()));

    let input = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {:?}", args.input))?;

    let cb = Box::leak(Box::new(broken_link_callback_with_replacers(
        replacers.clone(),
    )));

    // Enrich broken links with references
    let iterator = process_broken_links(&input, replacers.clone(), cb);
    let iterator = process_links(iterator, &replacers);

    let mut buf = String::with_capacity(input.len());
    let _state = cmark(iterator, &mut buf)?;

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
