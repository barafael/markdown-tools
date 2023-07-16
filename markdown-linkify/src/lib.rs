pub mod aggregator;
mod link;
mod transform;

use aggregator::Aggregation;
pub use transform::*;

use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LinkMetadata {
    destination: String,
    text: Option<String>,
    is_code: bool,
    title: Option<String>,
}

pub fn linkify(input: &str, replacers: &[Box<dyn LinkTransformer>]) -> anyhow::Result<String> {
    let parser = Parser::new(input);

    let mut metadata = None;

    let i = parser
        .map(|event| process_replacement(event, &mut metadata, replacers))
        .collect::<anyhow::Result<Vec<Vec<Event<'_>>>>>()?
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();

    let mut buf = String::with_capacity(input.len());
    let _state = cmark(&mut i.iter(), &mut buf)?;
    Ok(buf)
}

fn process_replacement<'a>(
    event: Event<'a>,
    metadata: &mut Option<LinkMetadata>,
    replacers: &[Box<dyn LinkTransformer>],
) -> anyhow::Result<Vec<Event<'a>>> {
    match event {
        ref event @ Event::Start(Tag::Link(ref _item_type, ref destination, ref title)) => {
            // Reset metadata
            let metadata = metadata.insert(LinkMetadata::default());
            // Record destination and title from start event
            metadata.destination = destination.to_string();
            metadata.title = Some(title.to_string());
            // Return unmodified start event
            Ok(vec![event.clone()])
        }
        Event::Text(ref text) => {
            if let Some(metadata) = metadata {
                // Set the metadata text for use on end event
                metadata.text = Some(text.to_string());
                Ok(vec![])
            } else {
                // If there is no metadata, pass through the event
                Ok(vec![event.clone()])
            }
        }
        Event::Code(ref text) => {
            if let Some(metadata) = metadata {
                // Set the metadata text for use on end event
                metadata.text = Some(text.to_string());
                metadata.is_code = true;
                Ok(vec![])
            } else {
                // If there is no metadata, pass through the event
                Ok(vec![event.clone()])
            }
        }
        Event::End(Tag::Link(item_type, ref destination, ref title)) => {
            if let Some(mut meta) = metadata.take() {
                apply_replacement(&mut meta, replacers)?;
                let title = meta.title.unwrap_or_else(|| title.to_string());
                let text = meta.text.unwrap_or_default().into();
                let text = if meta.is_code {
                    Event::Code(text)
                } else {
                    Event::Text(text)
                };
                Ok(vec![
                    text,
                    Event::End(Tag::Link(item_type, meta.destination.into(), title.into())),
                ])
            } else {
                Ok(vec![Event::End(Tag::Link(
                    item_type,
                    destination.clone(),
                    title.clone(),
                ))])
            }
        }
        event => Ok(vec![event.clone()]),
    }
}

#[derive(Debug, Default)]
pub struct Linkify<'a, I> {
    state: aggregator::Aggregator<'a>,
    iter: I,
}

impl<'a, I> Iterator for Linkify<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Aggregation<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if let Some(next) = next {
            let aggregation = self.state.push(next);
            aggregation
        } else {
            self.state.flush()
        }
    }
}

impl<'a, I> Linkify<'a, I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            state: aggregator::Aggregator::default(),
        }
    }
}

fn apply_replacement(
    meta: &mut LinkMetadata,
    config: &[Box<dyn LinkTransformer>],
) -> anyhow::Result<()> {
    for replacement in config {
        if !replacement.pattern().is_match(&meta.destination) {
            continue;
        }
        replacement.apply(meta)?;
    }
    Ok(())
}
