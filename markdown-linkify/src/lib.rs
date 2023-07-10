mod transform;

pub use transform::*;

use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LinkMetadata {
    destination: String,
    text: Option<String>,
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
        Event::End(Tag::Link(item_type, ref destination, ref title)) => {
            if let Some(mut meta) = metadata.take() {
                apply_replacement(&mut meta, replacers)?;
                let title = meta.title.unwrap_or_else(|| title.to_string());
                Ok(vec![
                    Event::Text(meta.text.unwrap_or_default().into()),
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
