use std::fmt::Debug;

use ::regex::Regex;

use crate::LinkMetadata;

pub mod docs_rustlang_replacer;
pub mod docsrs_replacer;
pub mod regex;

pub trait LinkTransformer: Debug {
    fn apply(&self, metadata: &mut LinkMetadata) -> anyhow::Result<()>;
    fn pattern(&self) -> Regex;
}
