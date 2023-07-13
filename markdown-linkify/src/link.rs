use std::vec::IntoIter;

use pulldown_cmark::{CowStr, Event, LinkType, Tag};

#[derive(Debug, Clone)]
pub struct Link<'a> {
    pub link_type: LinkType,
    pub destination: CowStr<'a>,
    pub title: CowStr<'a>,
    pub text: Vec<Event<'a>>,
}

impl<'a> Link<'a> {
    pub fn into_iter(mut self) -> IntoIter<Event<'a>> {
        let start = Event::Start(Tag::Link(
            self.link_type,
            self.destination.clone(),
            self.title.clone(),
        ));
        let end = Event::Start(Tag::Link(
            self.link_type,
            self.destination.clone(),
            self.title.clone(),
        ));
        self.text.insert(0, start);
        self.text.push(end);
        self.text.into_iter()
    }
}
