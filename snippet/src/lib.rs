use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

pub type Snippets = HashMap<String, Snippet>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snippet {
    pub content: String,
    pub file: PathBuf,
    pub line: usize,
    pub col: usize,
}
