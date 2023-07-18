use crate::aggregation::{Aggregation, Aggregator};
use crate::link::Link;

use pulldown_cmark::{Event, Tag};

#[derive(Debug, Default)]
pub struct LinkAggregator<'a, I> {
    state: Aggregator<'a>,
    iter: I,
}

impl<'a, I> LinkAggregator<'a, I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            state: Aggregator::default(),
        }
    }
}

impl<'a, I> Iterator for LinkAggregator<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Aggregation<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next();
            let Some(next) = next else {
                return match std::mem::replace(&mut self.state, Aggregator::Empty) {
                    Aggregator::Empty => None,
                    Aggregator::Start(link_type, destination, title) => {
                        Some(Aggregation::Link(Link {
                            link_type,
                            destination,
                            title,
                            text: vec![],
                        }))
                    }
                    Aggregator::Text(link) => Some(Aggregation::Link(link)),
                };
            };
            match (&self.state, next.clone()) {
                (Aggregator::Empty, Event::Start(Tag::Link(link_type, destination, title))) => {
                    self.state = Aggregator::Start(link_type, destination, title);
                    continue;
                }
                (Aggregator::Empty, e) => break Some(Aggregation::Event(e)),
                (Aggregator::Start(link_type, destination, title), e @ Event::Start(..)) => {
                    let start =
                        Event::Start(Tag::Link(*link_type, destination.clone(), title.clone()));
                    let agg = Aggregation::Bag(vec![start, e]);
                    self.state = Aggregator::Empty;
                    break Some(agg);
                }
                (Aggregator::Start(link_type, destination, title), Event::End(Tag::Link(..))) => {
                    let result = Link {
                        link_type: *link_type,
                        destination: destination.clone(),
                        title: title.clone(),
                        text: vec![],
                    };
                    self.state = Aggregator::Empty;
                    break Some(Aggregation::Link(result));
                }
                (
                    Aggregator::Start(link_type, destination, title),
                    e @ (Event::Text(..) | Event::Code(..)),
                ) => {
                    let link = Link {
                        link_type: *link_type,
                        destination: destination.clone(),
                        title: title.clone(),
                        text: vec![e],
                    };
                    self.state = Aggregator::Text(link);
                    continue;
                }
                (Aggregator::Text(link), e @ (Event::Text(..) | Event::Code(..))) => {
                    let mut new_text = link.clone();
                    new_text.text.push(e);
                    self.state = Aggregator::Text(new_text);
                    continue;
                }
                (Aggregator::Text(link), Event::End(Tag::Link(..))) => {
                    let result = link.clone();
                    self.state = Aggregator::Empty;
                    break Some(Aggregation::Link(result));
                }
                (_state, event) => break Some(Aggregation::Event(event)),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::aggregation::Aggregation;
    use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Options, Parser};

    #[test]
    fn aggregates_md() {
        let md = "# HEADING\n[simple](link \"right?\")\n## more heading";
        let parser = pulldown_cmark::Parser::new(md);

        let linkify = LinkAggregator::new(parser);
        linkify.for_each(|e| {
            dbg!(e);
        });
    }

    #[test]
    fn aggregates_simple_link() {
        let md = "[simple](link \"right?\")";
        let parser = Parser::new(md);

        let linkify = LinkAggregator::new(parser);
        for agg in linkify {
            let Aggregation::Link(link) = agg else {
                continue;
            };
            assert_eq!(link.text, vec![Event::Text("simple".into())]);
            assert_eq!(link.destination, "link".into());
            assert_eq!(link.title, "right?".into());
            assert_eq!(link.link_type, LinkType::Inline);
            return;
        }
        assert!(false);
    }

    #[test]
    fn aggregates_empty_code() {
        let md = "[``](thing \"titleee?\")";
        let parser = Parser::new(md);

        let linkify = LinkAggregator::new(parser);
        for agg in linkify {
            let Aggregation::Link(link) = agg else {
                continue;
            };
            assert_eq!(link.text, vec![Event::Text("``".into())]);
            assert_eq!(link.destination, "thing".into());
            assert_eq!(link.title, "titleee?".into());
            assert_eq!(link.link_type, LinkType::Inline);
        }
    }

    #[test]
    fn aggregates_test_file() {
        let md = include_str!("../test.md");
        let parser = Parser::new(md);

        let linkify = LinkAggregator::new(parser);
        linkify.for_each(|elem| {
            dbg!(elem);
        });
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
        let parser = Parser::new(md);
        let mut parser2 = Parser::new(md);

        let linkify = LinkAggregator::new(parser);
        for agg in linkify {
            for elem in agg.into_iter() {
                assert_eq!(Some(elem), parser2.next());
            }
        }
    }
}
