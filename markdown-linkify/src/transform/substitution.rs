use pulldown_cmark::Event;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{link::Link, LinkTransformer};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Substitution {
    #[serde(with = "serde_regex")]
    pattern: Regex,
    replacement: String,
    limit: usize,
    #[serde(default)]
    code: bool,
}

impl Substitution {
    pub fn example() -> Self {
        Self {
            pattern: regex::Regex::new(r"PS-(?<s>\d+)").expect("Invalid example regex"),
            replacement: "jira.com/issues/PS-$s".to_string(),
            limit: 3,
            code: false,
        }
    }
}

impl LinkTransformer for Substitution {
    fn pattern(&self) -> Regex {
        self.pattern.clone()
    }

    fn apply(&self, link: &mut Link) -> anyhow::Result<()> {
        let snippet = &self
            .pattern
            .replacen(&link.destination, self.limit, &self.replacement);
        let text = if let Some(caps) = self.pattern.captures(&link.destination) {
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
