use anyhow::Context;
use clap::Parser as ClapParser;
use pulldown_cmark::{BrokenLink, CowStr, Options, Parser};
use pulldown_cmark_to_cmark::cmark;
use std::path::PathBuf;
use std::{fs, io::Write};

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    #[arg()]
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, default_value_t = false)]
    dump: bool,
}

pub fn make_callback<'a>() -> impl Fn(BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
    move |link: BrokenLink<'a>| Some((link.reference, "".into()))
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let input = fs::read_to_string(args.input).context("Failed to read input file")?;

    let mut callback = make_callback();

    let mut parser =
        Parser::new_with_broken_link_callback(&input, Options::empty(), Some(&mut callback))
            .inspect(|elem| {
                if args.dump {
                    dbg!(elem);
                }
            });

    let mut buf = String::with_capacity(input.len());
    let _state = cmark(&mut parser, &mut buf)?;

    if let Some(path) = args.output {
        std::fs::write(path, buf)?;
    } else {
        let mut stdout = std::io::stdout();
        stdout.write_all(buf.as_bytes())?;
    }

    Ok(())
}
