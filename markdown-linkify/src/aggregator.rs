use std::vec::IntoIter;

use pulldown_cmark::{CowStr, Event, LinkType, Tag};

use crate::link::Link;

#[derive(Debug)]
pub enum Aggregation<'a> {
    Event(Event<'a>),
    Bag(Vec<Event<'a>>),
    Link(Link<'a>),
}

impl<'a> Aggregation<'a> {
    pub fn into_iter(self) -> IntoIter<Event<'a>> {
        match self {
            Aggregation::Event(e) => vec![e].into_iter(),
            Aggregation::Bag(vec) => vec.into_iter(),
            Aggregation::Link(l) => l.into_iter(),
        }
    }
}

#[derive(Debug, Default)]
pub enum Aggregator<'a> {
    #[default]
    Empty,
    Start(LinkType, CowStr<'a>, CowStr<'a>),
    Text(Link<'a>),
}

impl<'a> Aggregator<'a> {
    pub fn push(&'a mut self, event: Event<'a>) -> Option<Aggregation<'a>> {
        match (&*self, event) {
            (Aggregator::Empty, Event::Start(Tag::Link(link_type, destination, title))) => {
                *self = Self::Start(link_type, destination, title);
                None
            }
            (Aggregator::Empty, e) => Some(Aggregation::Event(e)),
            (Aggregator::Start(link_type, destination, title), e @ Event::Start(..)) => {
                let start = Event::Start(Tag::Link(*link_type, destination.clone(), title.clone()));
                let agg = Aggregation::Bag(vec![start, e]);
                *self = Self::Empty;
                Some(agg)
            }
            (Aggregator::Start(link_type, destination, title), Event::End(Tag::Link(..))) => {
                let result = Link {
                    link_type: *link_type,
                    destination: destination.clone(),
                    title: title.clone(),
                    text: vec![],
                };
                *self = Self::Empty;
                Some(Aggregation::Link(result))
            }
            (Aggregator::Start(link_type, destination, title), e @ Event::Text(..)) => {
                let link = Link {
                    link_type: *link_type,
                    destination: destination.clone(),
                    title: title.clone(),
                    text: vec![e],
                };
                *self = Self::Text(link);
                None
            }
            (Aggregator::Text(link), e @ Event::Text(..)) => {
                let mut new_text = link.clone();
                new_text.text.push(e);
                *self = Self::Text(new_text);
                None
            }
            (Aggregator::Text(link), e @ Event::Code(..)) => {
                let mut new_text = link.clone();
                new_text.text.push(e);
                *self = Self::Text(new_text);
                None
            }
            (Aggregator::Text(link), Event::End(Tag::Link(..))) => {
                let result = link.clone();
                *self = Self::Empty;
                Some(Aggregation::Link(result))
            }
            (_state, event) => Some(Aggregation::Event(event)),
        }
    }

    pub fn flush(&'a mut self) -> Option<Aggregation<'a>> {
        match self {
            Aggregator::Empty => None,
            Aggregator::Start(link_type, destination, title) => Some(Aggregation::Link(Link {
                link_type: *link_type,
                destination: destination.clone(),
                title: title.clone(),
                text: vec![],
            })),
            Aggregator::Text(link) => Some(Aggregation::Link(link.clone())),
        }
    }
}
