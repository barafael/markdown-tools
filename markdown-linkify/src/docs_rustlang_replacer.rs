use regex::Regex;
use tempdir::TempDir;

use crate::{LinkMetadata, Replacer};

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
    fn pattern(&self) -> Regex {
        Regex::new(r"rust:(?<i>.+)").unwrap()
    }

    fn apply(&self, meta: &mut LinkMetadata, snippet: &str) -> anyhow::Result<()> {
        let tmp_dir = TempDir::new("rustdoc-temp")?;
        let test_file_path = tmp_dir.path().join("snippet.rs");
        std::fs::write(&test_file_path, format!("/// [{snippet}]\npub struct X;"))?;

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

        let html = std::fs::read_to_string(result_file_path)?;
        let regex = Regex::new(r###"(?<l>https://doc.rust-lang.org/[^"]+)""###).unwrap();
        let (_full, [link]) = regex.captures(html.as_str()).unwrap().extract();

        meta.title = Some(snippet.to_string());
        meta.text = Some(snippet.to_string());
        meta.destination = link.to_string();
        Ok(())
    }
}
