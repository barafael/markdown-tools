use pulldown_cmark::{CowStr, Event, LinkType, Tag};

use crate::link::Link;

#[derive(Debug, Default)]
enum AggregatorState<'a> {
    #[default]
    Empty,
    Start(LinkType, CowStr<'a>, CowStr<'a>),
    Text(Link<'a>),
}

#[derive(Debug)]
enum Aggregation<'a> {
    Event(Event<'a>),
    Bag(Vec<Event<'a>>),
    Link(Link<'a>),
}

impl<'a> AggregatorState<'a> {
    fn push(&mut self, event: Event<'a>) -> Option<Aggregation> {
        match (&*self, event) {
            (AggregatorState::Empty, Event::Start(Tag::Link(link_type, destination, title))) => {
                *self = Self::Start(link_type, destination, title);
                None
            }
            (AggregatorState::Empty, e) => Some(Aggregation::Event(e)),
            (AggregatorState::Start(link_type, destination, title), e @ Event::Start(..)) => {
                let start = Event::Start(Tag::Link(*link_type, destination.clone(), title.clone()));
                let agg = Aggregation::Bag(vec![start, e]);
                *self = Self::Empty;
                Some(agg)
            }
            (AggregatorState::Start(link_type, destination, title), Event::End(Tag::Link(..))) => {
                let result = Link {
                    link_type: *link_type,
                    destination: destination.clone(),
                    title: title.clone(),
                    text: vec![],
                };
                *self = Self::Empty;
                Some(Aggregation::Link(result))
            }
            (AggregatorState::Start(link_type, destination, title), e @ Event::Text(..)) => {
                let link = Link {
                    link_type: *link_type,
                    destination: destination.clone(),
                    title: title.clone(),
                    text: vec![e],
                };
                *self = Self::Text(link);
                None
            }
            (AggregatorState::Text(link), e @ Event::Text(..)) => {
                let mut new_text = link.clone();
                new_text.text.push(e);
                *self = Self::Text(new_text);
                None
            }
            (AggregatorState::Text(link), e @ Event::Code(..)) => {
                let mut new_text = link.clone();
                new_text.text.push(e);
                *self = Self::Text(new_text);
                None
            }
            (AggregatorState::Text(link), Event::End(Tag::Link(..))) => {
                let result = link.clone();
                *self = Self::Empty;
                Some(Aggregation::Link(result))
            }
            (state, event) => Some(Aggregation::Event(event)),
        }
    }

    fn flush(self) -> Option<Link<'a>> {
        match self {
            AggregatorState::Empty => None,
            AggregatorState::Start(link_type, destination, title) => Some(Link {
                link_type,
                destination,
                title,
                text: vec![],
            }),
            AggregatorState::Text(link) => Some(link),
        }
    }
}

#[cfg(test)]
mod test {
    use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Options, Parser};

    use crate::aggregator::Aggregation;

    use super::AggregatorState;

    #[test]
    fn aggregates_simple_link() {
        let md = "[simple](link \"right?\")";
        let mut parser = Parser::new(md);

        let mut state = AggregatorState::default();
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

    fn aggregates_empty_code() {
        let md = "[``](thing \"titleee?\")";
        let mut parser = Parser::new(md);

        let mut state = AggregatorState::default();
        while let Some(event) = parser.next() {
            let Some(Aggregation::Link(link)) = state.push(event) else {
                continue;
            };
            assert_eq!(link.text, vec![Event::Code("".into())]);
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

        let mut state = AggregatorState::default();
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
        let mut parser = Parser::new_with_broken_link_callback(md, Options::empty(), Some(cb));
        parser.for_each(drop);
    }

    #[test]
    fn iterate_over_everything() {
        let md = "# HEADING\n[simple](link \"right?\")";
        let mut parser = Parser::new(md);

        let mut state = AggregatorState::default();
        while let Some(event) = parser.next() {
            let Some(aggregation) = state.push(event) else {
                continue;
            };
            dbg!(aggregation);
        }
        assert!(state.flush().is_none());
    }
}
