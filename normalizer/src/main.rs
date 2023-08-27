use clap::Parser as ClapParser;
use pulldown_cmark::Parser;
use pulldown_cmark_to_cmark::cmark;
use std::path::PathBuf;
use std::{fs, io::Write};

#[derive(Debug, Clone, ClapParser)]
struct Arguments {
    #[arg()]
    markdown_file: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, default_value_t = false)]
    dump: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let input = fs::read_to_string(args.markdown_file).unwrap();

    let mut parser = Parser::new(&input).inspect(|elem| {
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
