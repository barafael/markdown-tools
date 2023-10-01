use pulldown_cmark::Event;

use crate::link::Link;

use super::LinkReplacer;

const EMPTY_PLAYGROUND_IFRAME: &str = include_str!("empty_playground_iframe.html");

#[derive(Debug, Clone, Default)]
pub struct EmptyPlaygroundInserter;

impl EmptyPlaygroundInserter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LinkReplacer for EmptyPlaygroundInserter {
    fn tag(&self) -> String {
        String::from("empty_playground")
    }

    /// Access the constructed page, then get its html title.
    fn apply(&self, link: &mut Link) -> anyhow::Result<Event<'_>> {
        Ok(Event::Html(EMPTY_PLAYGROUND_IFRAME.into()))
    }
}
