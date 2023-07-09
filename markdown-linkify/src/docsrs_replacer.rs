use crate::LinkMetadata;
use crate::Replacer;
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

    fn apply(&self, meta: &mut LinkMetadata, snippet: &str) -> anyhow::Result<()> {
        let page = self.client.get(snippet).send().unwrap();
        let doc = Document::from(page.text().unwrap().as_str());
        let title = doc.find(Name("title")).next().unwrap();
        let name = title
            .first_child()
            .unwrap()
            .as_text()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();
        meta.title = Some(name.clone());
        meta.text = Some(name);
        meta.destination = snippet.to_string();
        Ok(())
    }
}
