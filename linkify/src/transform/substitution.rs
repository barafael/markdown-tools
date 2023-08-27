use pulldown_cmark::{Event, LinkType};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{link::Link, LinkTransformer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Substitution {
    #[serde(with = "serde_regex")]
    tail: Regex,
    tag: String,
    replacement: String,
    #[serde(default = "one")]
    limit: usize,
    #[serde(default)]
    code: bool,
}

/// Workaround for https://github.com/serde-rs/serde/issues/368
fn one() -> usize {
    1
}

impl Substitution {
    pub fn example() -> Self {
        Self {
            tail: regex::Regex::new(r"\d+").expect("Invalid example regex"),
            tag: String::from("PS-"),
            replacement: "mycompany.jira.com/issues/PS-$text".to_string(),
            limit: 1,
            code: false,
        }
    }
}

/// A [`Substitution`] is a link transformer of sorts, too.
impl LinkTransformer for Substitution {
    fn tag(&self) -> String {
        self.tag.clone()
    }

    fn pattern(&self) -> Regex {
        Regex::new(format!("(?<text>{}{})", self.tag(), self.tail).as_str()).expect("Invalid regex")
    }

    fn strip_tag(&self) -> bool {
        false
    }

    /// Perform the replacement.
    fn apply(&self, link: &mut Link) -> anyhow::Result<()> {
        if link.link_type == LinkType::ShortcutUnknown {
            return Ok(());
        }
        let snippet = &self
            .pattern()
            .replacen(&link.destination, self.limit, &self.replacement);
        let text = if let Some(caps) = self.tail.captures(&link.destination) {
            if let Some(text) = caps.name("text") {
                text.as_str().to_string()
            } else {
                snippet.to_string()
            }
        } else {
            snippet.to_string()
        };
        link.destination = snippet.to_string().into();
        if link.text.is_empty() {
            link.text = vec![Event::Text(text.into())];
        }
        if link.title.is_empty() {
            link.title = link.destination.clone();
        }
        Ok(())
    }
}
