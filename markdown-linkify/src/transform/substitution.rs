use pulldown_cmark::Event;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{link::Link, LinkTransformer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Substitution {
    #[serde(with = "serde_regex")]
    tail: Regex,
    tag: String,
    replacement: String,
    limit: usize,
    #[serde(default)]
    code: bool,
}

impl Substitution {
    pub fn example() -> Self {
        Self {
            tail: regex::Regex::new(r"(?<s>\d+)").expect("Invalid example regex"),
            tag: String::from("PS-"),
            replacement: "jira.com/issues/PS-$s".to_string(),
            limit: 3,
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
