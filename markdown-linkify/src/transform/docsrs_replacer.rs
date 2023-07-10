use crate::{LinkMetadata, Replacer};
use anyhow::Context;
use regex::Regex;
use select::document::Document;
use select::predicate::Name;

#[derive(Debug, Clone, Default)]
pub struct DocsrsReplacer {
    client: reqwest::blocking::Client,
}

impl DocsrsReplacer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Replacer for DocsrsReplacer {
    fn pattern(&self) -> Regex {
        Regex::new(r"docsrs:(?<i>.+)").unwrap()
    }

    fn apply(&self, meta: &mut LinkMetadata) -> anyhow::Result<()> {
        let url = self
            .pattern()
            .replacen(&meta.destination, 1, "$i")
            .to_string();
        let page = self
            .client
            .get(url.clone())
            .send()
            .with_context(|| format!("Failed to access {url}"))?;
        let doc = Document::from(
            page.text()
                .with_context(|| format!("Failed to parse document at {url}"))?
                .as_str(),
        );
        let title = doc
            .find(Name("title"))
            .next()
            .with_context(|| format!("Failed to get title of {url}"))?;
        let name = title
            .first_child()
            .context("First child of title node not found")?
            .as_text()
            .context("No title set")?
            .split_whitespace()
            .next()
            .context("Can't split first word of title")?
            .to_string();
        meta.title = Some(name.clone());
        meta.text = Some(name);
        meta.destination = url.to_string();
        Ok(())
    }
}
