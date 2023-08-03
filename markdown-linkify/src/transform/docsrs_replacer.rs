use crate::link::Link;
use crate::LinkTransformer;
use anyhow::Context;
use pulldown_cmark::Event;
use select::document::Document;
use select::predicate::Name;

#[derive(Debug, Clone, Default)]
pub struct Docsrs {
    client: reqwest::blocking::Client,
}

impl Docsrs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LinkTransformer for Docsrs {
    fn tag(&self) -> String {
        String::from("docsrs:")
    }

    /// Access the constructed page, then get its html title.
    fn apply(&self, link: &mut Link) -> anyhow::Result<()> {
        let url = self
            .pattern()
            .replacen(&link.destination, 1, "$i")
            .to_string();
        let page = self
            .client
            .get(&url)
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

        link.destination = url.to_string().into();
        if link.title.is_empty() {
            link.title = url.into();
        }
        link.text = vec![Event::Text(name.into())];
        Ok(())
    }
}
