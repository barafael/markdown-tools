use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::LinkTransformer;

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

    fn apply(&self, metadata: &mut crate::LinkMetadata) -> anyhow::Result<()> {
        let snippet = self
            .pattern
            .replacen(&metadata.destination, self.limit, &self.replacement);
        let text = if let Some(caps) = self.pattern.captures(&metadata.destination) {
            if let Some(text) = caps.name("text") {
                text.as_str().to_string()
            } else {
                snippet.to_string()
            }
        } else {
            snippet.to_string()
        };
        metadata.destination = snippet.to_string();
        if metadata.text.is_none() {
            metadata.text = Some(text);
        }
        if metadata.title.is_none() || metadata.title == Some(String::new()) {
            metadata.title = Some(metadata.destination.clone());
        }
        metadata.is_code = self.code;
        Ok(())
    }
}
