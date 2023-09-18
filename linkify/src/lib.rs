#![doc = include_str!("../README.md")]

use link_aggregator::LinkTools;
pub use transform::*;

use pulldown_cmark::{BrokenLink, CowStr, Event, Options, Parser};

use crate::aggregation::Aggregation;

pub mod aggregation;
pub mod link;
pub mod link_aggregator;
mod transform;

pub fn broken_link_callback_with_replacers<'a>(
    replacers: Vec<Box<dyn LinkTransformer>>,
) -> impl Fn(BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
    move |link: BrokenLink<'a>| {
        for replacer in &replacers {
            if replacer.pattern().is_match(&link.reference) {
                let mut link = link::Link {
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

pub fn process_broken_links<'a>(
    input: &'a str,
    replacers: Vec<Box<dyn LinkTransformer>>,
    cb: &'a mut impl Fn(BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)>,
) -> impl Iterator<Item = Event<'a>> {
    // Add missing references via the broken link callback (the replacers are registered there).
    let parser = Parser::new_with_broken_link_callback(input, Options::empty(), Some(cb));

    // transform `rust:Vec` to `Vec` (remove the tag text, if present).
    parser.aggregate_links().flat_map(move |mut aggregation| {
        // return links untouched.
        let Aggregation::Link(ref mut link) = aggregation else {
            return aggregation;
        };
        // return empty text and code events untouched.
        let Some(Event::Text(first)) = link.text.get_mut(0) else {
            return aggregation;
        };
        // turn broken links into full links
        for replacer in &replacers {
            // remove initial replacer tag (such as `rust:`)
            // TODO why `continue`?
            if replacer.strip_tag() {
                if first.starts_with(&replacer.tag()) {
                    let new_text = first.replace(&replacer.tag(), "");
                    *first = new_text.into();
                    return Aggregation::Link(link.clone());
                }
            }
        }
        aggregation
    })
}

pub fn process_links<'a>(
    input: impl Iterator<Item = Event<'a>>,
    replacers: &'a [Box<dyn LinkTransformer>],
) -> impl Iterator<Item = Event<'a>> {
    input
        .aggregate_links()
        .flat_map(move |aggregation| {
            let Aggregation::Link(mut link) = aggregation else {
                return anyhow::Ok(aggregation);
            };
            for replacement in replacers {
                if replacement.pattern().is_match(&link.destination) {
                    replacement.apply(&mut link)?;
                    break;
                }
            }
            Ok(Aggregation::Link(link))
        })
        .flatten()
}
