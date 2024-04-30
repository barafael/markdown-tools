pub(crate) use std::vec::IntoIter;

use pulldown_cmark::{CowStr, Event, LinkType, Tag, TagEnd};

/// A link with items as represented by [`pulldown_cmark::Event`].
/// Except, instead of separate events, this is one complete datastructure
/// suitable to run a replacer on.
#[derive(Debug, Clone)]
pub struct Link<'a> {
    pub link_type: LinkType,
    pub destination: CowStr<'a>,
    pub title: CowStr<'a>,
    pub text: Vec<Event<'a>>,
    pub id: CowStr<'a>,
}

impl<'a> IntoIterator for Link<'a> {
    type Item = Event<'a>;

    type IntoIter = IntoIter<Event<'a>>;

    fn into_iter(mut self) -> Self::IntoIter {
        let start = Event::Start(Tag::Link {
            link_type: self.link_type,
            dest_url: self.destination.clone(),
            title: self.title.clone(),
            id: "".into(),
        });
        let end = Event::End(TagEnd::Link);
        self.text.insert(0, start);
        self.text.push(end);
        self.text.into_iter()
    }
}
