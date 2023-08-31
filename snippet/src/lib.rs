use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use relative_path::RelativePathBuf;

pub type Snippets = HashMap<String, Snippet>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snippet {
    pub content: String,
    pub file: RelativePathBuf,
    pub line: usize,
    pub col: usize,
}
