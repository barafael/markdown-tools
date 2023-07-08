use std::fmt::Debug;

pub trait Replacer: Debug {
    fn apply(&self, snippet: &str) -> Option<String>;
}
