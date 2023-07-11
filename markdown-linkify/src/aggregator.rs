use pulldown_cmark::{CowStr, Event, LinkType, Tag};

#[derive(Debug, Clone)]
pub struct Link<'a> {
    link_type: LinkType,
    destination: CowStr<'a>,
    title: CowStr<'a>,
    text: Vec<Event<'a>>,
}

#[derive(Debug, Default)]
enum AggregatorState<'a> {
    #[default]
    Empty,
    Start(LinkType, CowStr<'a>, CowStr<'a>),
    Text(Link<'a>),
}

impl<'a> AggregatorState<'a> {
    fn push(&mut self, event: Event<'a>) -> Option<Link<'a>> {
        match (&*self, event) {
            (AggregatorState::Empty, Event::Start(Tag::Link(link_type, destination, title))) => {
                *self = Self::Start(link_type, destination, title);
                None
            }
            (AggregatorState::Empty, _e) => None,
            (AggregatorState::Start(..), Event::Start(tag)) => {
                eprintln!("Repeated start, ignoring {tag:?}");
                None
            }
            (AggregatorState::Start(link_type, destination, title), Event::End(Tag::Link(..))) => {
                let result = Some(Link {
                    link_type: *link_type,
                    destination: destination.clone(),
                    title: title.clone(),
                    text: vec![],
                });
                *self = Self::Empty;
                result
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
                let result = Some(link.clone());
                *self = Self::Empty;
                result
            }
            _ => None,
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
    use pulldown_cmark::{CowStr, Event, InlineStr, LinkType, Parser};

    use crate::aggregator::Link;

    use super::AggregatorState;

    #[test]
    fn aggregates_simple_link() {
        let md = "[simple](link \"right?\")";
        let mut parser = Parser::new(md);

        let mut state = AggregatorState::default();
        while let Some(event) = parser.next() {
            let Some(link) = state.push(event) else {
                continue;
            };
            assert_eq!(link.text, vec![Event::Text("simple".into())]);
            assert_eq!(link.destination, "link".into());
            assert_eq!(link.title, "right?".into());
            assert_eq!(link.link_type, LinkType::Inline);
        }
        assert!(state.flush().is_none());
    }
}
