use std::fmt::Debug;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::link::Link;

use self::substitution::Substitution;

pub mod docs_rustlang_replacer;
pub mod docsrs_replacer;
pub mod substitution;

/// If a link transformer matches on a tag, it shall produce a meaningful regex-substituted transformation.
pub trait LinkTransformer: Debug + dyn_clone::DynClone {
    /// Apply the link transformation.
    fn apply(&self, link: &mut Link) -> anyhow::Result<()>;

    /// Text on which the link transformer can operate shall start with this string.
    fn tag(&self) -> String;

    /// Regex which will be applied to extract the salient part of the link destination.
    fn pattern(&self) -> Regex {
        Regex::new(format!("{}(?<i>.+)", self.tag()).as_str()).expect("Invalid regex")
    }

    fn strip_tag(&self) -> bool {
        true
    }
}

dyn_clone::clone_trait_object!(LinkTransformer);

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Transformers {
    pub regexes: Vec<Substitution>,
}

impl Transformers {
    /// An example configuration, used to generate a template file.
    pub fn example() -> Self {
        Self {
            regexes: vec![Substitution::example()],
        }
    }
}
