use std::vec::IntoIter;

use pulldown_cmark::{CowStr, Event, LinkType, Tag};

#[derive(Debug, Clone)]
pub struct Link<'a> {
    pub link_type: LinkType,
    pub destination: CowStr<'a>,
    pub title: CowStr<'a>,
    pub text: Vec<Event<'a>>,
}

impl<'a> IntoIterator for Link<'a> {
    type Item = Event<'a>;

    type IntoIter = IntoIter<Event<'a>>;

    fn into_iter(mut self) -> Self::IntoIter {
        let start = Event::Start(Tag::Link(
            self.link_type,
            self.destination.clone(),
            self.title.clone(),
        ));
        let end = Event::End(Tag::Link(self.link_type, self.destination, self.title));
        self.text.insert(0, start);
        self.text.push(end);
        self.text.into_iter()
    }
}
