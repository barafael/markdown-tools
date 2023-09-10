use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};

pub use relative_path::RelativePathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Snippets {
    pub snippets: BTreeMap<PathBuf, BTreeMap<String, Snippet>>,
}

impl Snippets {
    pub fn snippets_for_id(&self, id: &str) -> Vec<Snippet> {
        let mut results = Vec::new();
        for snippets in self.snippets.values() {
            results.extend(snippets.get(id).iter().copied().cloned());
        }
        results
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Snippet {
    pub content: String,
    pub file: RelativePathBuf,
    pub line: usize,
    pub col: usize,
}
