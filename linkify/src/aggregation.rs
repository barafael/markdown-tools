use std::vec::IntoIter;

use pulldown_cmark::Event;

use crate::link::Link;

#[derive(Debug)]
pub enum Aggregation<'a> {
    Event(Event<'a>),
    Bag(Vec<Event<'a>>),
    Link(Link<'a>),
}

impl<'a> IntoIterator for Aggregation<'a> {
    type Item = Event<'a>;

    type IntoIter = IntoIter<Event<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Aggregation::Event(e) => vec![e].into_iter(),
            Aggregation::Bag(vec) => vec.into_iter(),
            Aggregation::Link(l) => l.into_iter(),
        }
    }
}
