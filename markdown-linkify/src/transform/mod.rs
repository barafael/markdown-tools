use std::fmt::Debug;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::link::Link;

use self::substitution::Substitution;

pub mod docs_rustlang_replacer;
pub mod docsrs_replacer;
pub mod substitution;

pub trait LinkTransformer: Debug {
    fn apply(&self, link: &mut Link) -> anyhow::Result<()>;
    // TODO should return &'str probably
    fn tag(&self) -> String;
    fn pattern(&self) -> Regex {
        Regex::new(format!("{}(?<i>.+)", self.tag()).as_str()).expect("Invalid regex")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transformers {
    pub regexes: Vec<Substitution>,
}

impl Transformers {
    pub fn example() -> Self {
        Self {
            regexes: vec![Substitution::example(), Substitution::example()],
        }
    }
}
