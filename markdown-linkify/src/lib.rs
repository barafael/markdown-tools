use link::Link;
use link_aggregator::LinkTools;
pub use transform::*;

use pulldown_cmark::Parser;
use pulldown_cmark_to_cmark::cmark;

use crate::aggregation::Aggregation;

mod aggregation;
mod link;
pub mod link_aggregator;
mod transform;

pub fn linkify(input: &str, replacers: &[Box<dyn LinkTransformer>]) -> anyhow::Result<String> {
    let parser = Parser::new(input);

    let i = parser
        .aggregate_links()
        .flat_map(|aggregation| {
            let Aggregation::Link(mut link) = aggregation else {
                return anyhow::Ok(aggregation);
            };
            process_replacement(&mut link, replacers)?;
            Ok(Aggregation::Link(link))
        })
        .flatten();

    let mut buf = String::with_capacity(input.len());
    let _state = cmark(i, &mut buf)?;
    Ok(buf)
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
