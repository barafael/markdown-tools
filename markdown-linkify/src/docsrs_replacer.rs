use crate::{replacer::LinkInfo, Replacer};
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
    fn apply(&self, snippet: &str) -> Option<LinkInfo> {
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
        Some(LinkInfo {
            title: Some(name),
            link: snippet.to_string(),
        })
    }

    fn pattern(&self) -> String {
        r"docsrs:(?<i>.+)".to_string()
    }
}
