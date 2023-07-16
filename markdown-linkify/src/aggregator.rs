use std::vec::IntoIter;

use pulldown_cmark::{CowStr, Event, LinkType, Tag};

use crate::link::Link;

#[derive(Debug)]
enum Aggregation<'a> {
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
enum Aggregator<'a> {
    #[default]
    Empty,
    Start(LinkType, CowStr<'a>, CowStr<'a>),
    Text(Link<'a>),
}

impl<'a> Aggregator<'a> {
    fn push(&mut self, event: Event<'a>) -> Option<Aggregation> {
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

    fn flush(self) -> Option<Link<'a>> {
        match self {
            Aggregator::Empty => None,
            Aggregator::Start(link_type, destination, title) => Some(Link {
                link_type,
                destination,
                title,
                text: vec![],
            }),
            Aggregator::Text(link) => Some(link),
        }
    }
}

#[cfg(test)]
mod test {
    use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Options, Parser};

    use crate::aggregator::Aggregation;

    use super::Aggregator;

    #[test]
    fn aggregates_simple_link() {
        let md = "[simple](link \"right?\")";
        let mut parser = Parser::new(md);

        let mut state = Aggregator::default();
        while let Some(event) = parser.next() {
            let Some(Aggregation::Link(link)) = state.push(event) else {
                continue;
            };
            assert_eq!(link.text, vec![Event::Text("simple".into())]);
            assert_eq!(link.destination, "link".into());
            assert_eq!(link.title, "right?".into());
            assert_eq!(link.link_type, LinkType::Inline);
        }
        assert!(state.flush().is_none());
    }

    #[test]
    fn aggregates_empty_code() {
        let md = "[``](thing \"titleee?\")";
        let mut parser = Parser::new(md);

        let mut state = Aggregator::default();
        while let Some(event) = parser.next() {
            let Some(Aggregation::Link(link)) = state.push(event) else {
                continue;
            };
            assert_eq!(link.text, vec![Event::Text("``".into())]);
            assert_eq!(link.destination, "thing".into());
            assert_eq!(link.title, "titleee?".into());
            assert_eq!(link.link_type, LinkType::Inline);
        }
        assert!(state.flush().is_none());
    }

    #[test]
    fn aggregates_test_file() {
        let md = include_str!("../test.md");
        let mut parser = Parser::new(md);

        let mut state = Aggregator::default();
        while let Some(event) = parser.next() {
            let Some(link) = state.push(event) else {
                continue;
            };
            dbg!(link);
        }
    }

    #[test]
    fn broken_link_callback() {
        fn callback<'a>(link: BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
            dbg!(&link);
            Some(("".into(), link.reference.into()))
        }
        let md = "[foo]";
        let cb = &mut callback;
        let parser = Parser::new_with_broken_link_callback(md, Options::empty(), Some(cb));
        parser.for_each(drop);
    }

    #[test]
    fn iterate_over_everything() {
        let md = "# HEADING\n[simple](link \"right?\")";
        let mut parser = Parser::new(md);

        let mut state = Aggregator::default();
        let mut parser2 = Parser::new(md);
        while let Some(event) = parser.next() {
            let Some(aggregation) = state.push(event) else {
                continue;
            };
            for elem in aggregation.into_iter() {
                assert_eq!(Some(elem), parser2.next());
            }
        }
        assert!(state.flush().is_none());
    }
}
