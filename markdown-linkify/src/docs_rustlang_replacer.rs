use regex::Regex;
use tempdir::TempDir;

use crate::{replacer::LinkInfo, Replacer};

#[derive(Debug, Clone, Default)]
pub struct DocsRustlangReplacer {
    _client: reqwest::Client,
}

impl DocsRustlangReplacer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Replacer for DocsRustlangReplacer {
    fn apply(&self, snippet: &str) -> Option<LinkInfo> {
        let tmp_dir = TempDir::new("rustdoc-temp").ok()?;
        let test_file_path = tmp_dir.path().join("snippet.rs");
        std::fs::write(&test_file_path, format!("/// [{snippet}]\npub struct X;")).ok()?;

        let output = std::process::Command::new("rustdoc")
            .arg("--out-dir")
            .arg(tmp_dir.path())
            .arg(test_file_path)
            .spawn()
            .expect("Failed to spawn rustdoc")
            .wait()
            .expect("Failed awaiting rustdoc result");

        if !output.success() {
            eprintln!("Rustdoc exited with error {output:?}");
        }

        let result_file_path = tmp_dir.path().join("snippet").join("index.html");

        let html = std::fs::read_to_string(result_file_path).ok()?;
        let regex = Regex::new(r###"(?<l>https://doc.rust-lang.org/[^"]+)""###).unwrap();
        let (_full, [link]) = regex.captures(html.as_str()).unwrap().extract();

        Some(LinkInfo {
            //title: Some(snippet.to_string()),
            title: None,
            link: link.to_string(),
        })
    }

    fn pattern(&self) -> String {
        r"rust:(?<i>.+)".to_string()
    }
}
