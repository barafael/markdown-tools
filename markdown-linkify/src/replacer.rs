use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkInfo {
    pub title: Option<String>,
    pub link: String,
}

pub trait Replacer: Debug {
    fn apply(&self, snippet: &str) -> Option<LinkInfo>;
    fn pattern(&self) -> String;
}
