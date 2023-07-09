use std::fmt::Debug;

use regex::Regex;

use crate::LinkMetadata;

pub trait Replacer: Debug {
    fn apply(&self, metadata: &mut LinkMetadata, snippet: &str) -> anyhow::Result<()>;
    fn pattern(&self) -> Regex;
}
