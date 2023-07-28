use anyhow::Context;
use clap::Parser as ClapParser;
use markdown_linkify::aggregation::Aggregation;
use markdown_linkify::docs_rustlang_replacer::DocsRustlang;
use markdown_linkify::docsrs_replacer::Docsrs;
use markdown_linkify::link_aggregator::LinkTools;
use markdown_linkify::{linkify, LinkTransformer, Transformers};
use pulldown_cmark::{BrokenLink, CowStr, Event};
use pulldown_cmark_to_cmark::cmark;
use std::path::PathBuf;
use std::{fs, io::Write};

use pulldown_cmark::{Options, Parser};

pub fn create_callback<'a>(
    replacers: Vec<Box<dyn LinkTransformer>>,
) -> impl Fn(BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
    move |link: BrokenLink<'a>| {
        for replacer in &replacers {
            if replacer.pattern().is_match(&link.reference) {
                let mut link = markdown_linkify::link::Link {
                    link_type: link.link_type,
                    destination: link.reference,
                    title: "".into(),
                    text: vec![],
                };
                replacer.apply(&mut link).unwrap();
                return Some((link.destination, link.title));
            }
        }
        None
    }
}

pub fn links<'a>(
    input: &'a str,
    replacers: Vec<Box<dyn LinkTransformer>>,
    cb: &'a mut impl Fn(BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)>,
) -> impl Iterator<Item = Event<'a>> {
    let parser = Parser::new_with_broken_link_callback(input, Options::empty(), Some(cb));
    parser.aggregate_links().flat_map(move |mut aggregation| {
        let Aggregation::Link(ref mut link) = aggregation else {
            return aggregation;
        };
        let Some(Event::Text(first)) = link.text.get_mut(0) else {
            return aggregation;
        };
        for replacer in &replacers {
            if first.starts_with(&replacer.tag()) {
                let new_text = first.replace(&replacer.tag(), "");
                *first = new_text.into();
                return Aggregation::Link(link.clone());
            }
        }
        aggregation
    })
}

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

    let regex_replacers: Transformers =
        if let Ok(regex_replacers) = fs::read_to_string(&args.config) {
            toml::from_str(&regex_replacers)?
        } else {
            Transformers { regexes: vec![] }
        };

    let mut replacers: Vec<Box<dyn LinkTransformer>> = Vec::new();

    for replacer in regex_replacers.regexes {
        replacers.push(Box::new(replacer));
    }
    replacers.push(Box::new(DocsRustlang::new()));
    replacers.push(Box::new(Docsrs::new()));

    let input = fs::read_to_string(&args.input)
        .with_context(|| format!("Failed to read input file: {:?}", args.input))?;

    let cb = Box::leak(Box::new(create_callback(replacers.clone())));
    let iterator = links(&input, replacers.clone(), cb);
    let iterator = linkify(iterator, &replacers);

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
