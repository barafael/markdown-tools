use config::Replacement;

pub use replacer::Replacer;

pub mod config;
pub mod docs_rustlang_replacer;
mod replacer;

use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark_to_cmark::cmark;

pub fn linkify(input: &str, config: &config::Config) -> anyhow::Result<String> {
    let parser = Parser::new(input);

    let mut in_empty_link = false;
    let mut current_link = None;

    let mut i = parser.map(|event| match event {
        Event::Start(Tag::Link(ref item_type, ref destination, ref title)) => {
            if destination.is_empty() {
                in_empty_link = true;
            }
            Event::Start(Tag::Link(*item_type, destination.clone(), title.clone()))
        }
        Event::Text(ref text) if in_empty_link => {
            let mut text = text.clone();
            for replacement in &config.replacements {
                match replacement {
                    Replacement::Regex {
                        pattern,
                        replacement,
                        limit,
                    } => {
                        if pattern.is_match(&text) {
                            let result = pattern.replacen(&text, *limit, replacement);
                            current_link = Some(result.to_string());
                            break;
                        }
                    }
                    Replacement::Replacer { pattern, replacer } => {
                        let result = if pattern.is_match(&text) {
                            let result = pattern.replacen(&text, 1, "$i");
                            if let Some(new) = replacer.apply(result.to_string().as_str()) {
                                current_link = Some(new.to_string());
                            }
                            result.to_string()
                        } else {
                            break;
                        };
                        text = result.into();
                    }
                }
            }
            let new_text = text.to_string();
            Event::Text(new_text.into())
        }
        Event::End(Tag::Link(item_type, destination, ref title)) if in_empty_link => {
            in_empty_link = false;
            let dest = current_link
                .take()
                .unwrap_or_else(|| destination.into_string());
            Event::End(Tag::Link(item_type, dest.into(), title.clone()))
        }
        event => event,
    });

    let mut buf = String::with_capacity(input.len());
    let _state = cmark(&mut i, &mut buf)?;
    Ok(buf)
}
