use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Replacer;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegexReplacer {
    #[serde(with = "serde_regex")]
    pattern: Regex,
    replacement: String,
    limit: usize,
}

impl RegexReplacer {
    pub fn example() -> Self {
        RegexReplacer {
            pattern: regex::Regex::new(r"PS-(?<s>\d+)").unwrap(),
            replacement: "jira.com/issues/PS-$s".to_string(),
            limit: 3,
        }
    }
}

impl Replacer for RegexReplacer {
    fn pattern(&self) -> Regex {
        self.pattern.clone()
    }

    fn apply(&self, metadata: &mut crate::LinkMetadata) -> anyhow::Result<()> {
        let snippet = self
            .pattern
            .replacen(&metadata.destination, self.limit, &self.replacement)
            .clone();
        metadata.destination = snippet.to_string();
        if metadata.text.is_none() {
            metadata.text = Some(metadata.destination.clone());
        }
        if metadata.title.is_none() {
            metadata.title = Some(metadata.destination.clone());
        }
        Ok(())
    }
}
