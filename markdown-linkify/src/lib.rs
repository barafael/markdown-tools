use link::Link;
use link_aggregator::LinkTools;
pub use transform::*;

use pulldown_cmark::Event;

use crate::aggregation::Aggregation;

pub mod aggregation;
pub mod link;
pub mod link_aggregator;
mod transform;

pub fn linkify<'a>(
    input: impl Iterator<Item = Event<'a>>,
    replacers: &'a [Box<dyn LinkTransformer>],
) -> impl Iterator<Item = Event<'a>> {
    input
        .aggregate_links()
        .flat_map(|aggregation| {
            let Aggregation::Link(mut link) = aggregation else {
                return anyhow::Ok(aggregation);
            };
            process_replacement(&mut link, replacers)?;
            Ok(Aggregation::Link(link))
        })
        .flatten()
}

fn process_replacement(
    link: &mut Link,
    replacers: &[Box<dyn LinkTransformer>],
) -> anyhow::Result<()> {
    for replacement in replacers {
        if !replacement.pattern().is_match(&link.destination) {
            continue;
        }
        replacement.apply(link)?;
        break;
    }
    Ok(())
}
