use pulldown_cmark::Event;

use crate::link::Link;

use std::fmt::Debug;

pub mod empty_playground_inserter;

pub trait LinkReplacer: Debug + dyn_clone::DynClone {
    fn apply(&self, link: &mut Link) -> anyhow::Result<Event<'_>>;

    fn tag(&self) -> String;
}
