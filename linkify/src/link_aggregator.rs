use crate::aggregation::Aggregation;
use crate::link::Link;

use pulldown_cmark::{CowStr, Event, LinkType, Tag, TagEnd};

#[derive(Debug, Default)]
pub enum Aggregator<'a> {
    #[default]
    Empty,
    Start(LinkType, CowStr<'a>, CowStr<'a>, CowStr<'a>),
    Text(Link<'a>),
}

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

/// Walk over an iterator of [`Event`]s.
/// On encountering a start of a link, then some text, then an end of a link:
/// pass on an aggregation of the encountered link.
/// Otherwise, pass on all items.
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
                    Aggregator::Start(link_type, destination, title, id) => {
                        Some(Aggregation::Link(Link {
                            link_type,
                            destination,
                            title,
                            text: vec![],
                            id,
                        }))
                    }
                    Aggregator::Text(link) => Some(Aggregation::Link(link)),
                };
            };
            let state = std::mem::replace(&mut self.state, Aggregator::Empty);
            match (state, next) {
                (
                    Aggregator::Empty,
                    Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    }),
                ) => {
                    self.state = Aggregator::Start(link_type, dest_url, title, id);
                    continue;
                }
                (Aggregator::Empty, e) => break Some(Aggregation::Event(e)),
                (Aggregator::Start(link_type, dest_url, title, id), e @ Event::Start(..)) => {
                    let start = Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    });
                    let agg = Aggregation::Bag(vec![start, e]);
                    self.state = Aggregator::Empty;
                    break Some(agg);
                }
                (Aggregator::Start(link_type, dest_url, title, id), Event::End(TagEnd::Link)) => {
                    let result = Link {
                        link_type,
                        destination: dest_url,
                        title,
                        text: vec![],
                        id,
                    };
                    self.state = Aggregator::Empty;
                    break Some(Aggregation::Link(result));
                }
                (
                    Aggregator::Start(link_type, destination, title, id),
                    e @ (Event::Text(..) | Event::Code(..)),
                ) => {
                    let link = Link {
                        link_type,
                        destination,
                        title,
                        text: vec![e],
                        id,
                    };
                    self.state = Aggregator::Text(link);
                    continue;
                }
                (Aggregator::Text(mut link), e @ (Event::Text(..) | Event::Code(..))) => {
                    link.text.push(e);
                    self.state = Aggregator::Text(link);
                    continue;
                }
                (Aggregator::Text(link), Event::End(TagEnd::Link)) => {
                    self.state = Aggregator::Empty;
                    break Some(Aggregation::Link(link));
                }
                (_state, event) => break Some(Aggregation::Event(event)),
            }
        }
    }
}

pub trait LinkTools: Iterator {
    fn aggregate_links<'a>(self) -> LinkAggregator<'a, Self>
    where
        Self: Sized;
}

impl<T> LinkTools for T
where
    T: Iterator + ?Sized,
{
    fn aggregate_links<'a>(self) -> LinkAggregator<'a, Self>
    where
        Self: Sized,
    {
        LinkAggregator::new(self.into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pulldown_cmark::{BrokenLink, Options, Parser};
    use pulldown_cmark_to_cmark::cmark;

    #[test]
    fn aggregates_md() {
        let md = "# HEADING\n[simple](link \"right?\")\n## more heading";
        let parser = pulldown_cmark::Parser::new(md);
        parser.aggregate_links().for_each(|elem| {
            dbg!(elem);
        });
    }

    #[test]
    fn aggregates_simple_link() {
        let md = "[simple](link \"right?\")";
        let parser = Parser::new(md);

        for agg in parser.aggregate_links() {
            let Aggregation::Link(link) = agg else {
                continue;
            };
            assert_eq!(link.text, vec![Event::Text("simple".into())]);
            assert_eq!(link.destination, "link".into());
            assert_eq!(link.title, "right?".into());
            assert_eq!(link.link_type, LinkType::Inline);
            return;
        }
        panic!("Should return above");
    }

    #[test]
    fn aggregates_empty_code() {
        let md = "[``](thing \"titleee?\")";
        let parser = Parser::new(md);

        for agg in parser.aggregate_links() {
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
    fn broken_link_callback() {
        fn callback(link: BrokenLink) -> Option<(CowStr, CowStr)> {
            Some(("destination".into(), link.reference))
        }
        let md = "[foo `this` works `nicely`]";
        let cb = &mut callback;
        let parser = Parser::new_with_broken_link_callback(md, Options::empty(), Some(cb));

        let mut buf = String::new();
        let _state = cmark(parser, &mut buf).expect("CMark failed");
        println!("{buf}");
    }

    #[test]
    fn iterate_over_everything() {
        let md = "# HEADING\n[simple](link \"right?\")";
        let parser = Parser::new(md);
        let mut parser2 = Parser::new(md);

        for agg in parser.aggregate_links() {
            for elem in agg {
                assert_eq!(Some(elem), parser2.next());
            }
        }
    }

    #[test]
    fn empty_links() {
        let md = "[foo]\n# HEADING\n[foo]: /url \"title\"\n\n[foo]";
        let links = Parser::new(md).aggregate_links();
        links.for_each(|elem| {
            dbg!(elem);
        });
    }
}
