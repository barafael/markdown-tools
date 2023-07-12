use anyhow::Context;
use regex::Regex;
use tempfile::TempDir;

use crate::{LinkMetadata, LinkTransformer};

#[derive(Debug, Clone, Default)]
pub struct DocsRustlang {
    _client: reqwest::Client,
}

impl DocsRustlang {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LinkTransformer for DocsRustlang {
    fn pattern(&self) -> Regex {
        Regex::new(r"rust:(?<i>.+)").unwrap()
    }

    fn apply(&self, meta: &mut LinkMetadata) -> anyhow::Result<()> {
        // Extract item name
        let snippet = self
            .pattern()
            .replacen(&meta.destination, 1, "$i")
            .to_string();
        // Create temporary directory with rust file using our item in the docs
        let tmp_dir = TempDir::new()?;
        let test_file_path = tmp_dir.path().join("snippet.rs");
        std::fs::write(&test_file_path, format!("//! [{snippet}]"))?;

        // Invoke rustdoc for creating our docs
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

        // Read generated html
        let result_file_path = tmp_dir.path().join("snippet").join("index.html");
        let html = std::fs::read_to_string(result_file_path)?;

        // Find URL in generated html
        let regex = Regex::new(r#"(?<l>https://doc.rust-lang.org/[^"]+)""#).expect("Invalid regex");
        let (_full, [link]) = regex
            .captures(html.as_str())
            .with_context(|| format!("No captures found for {snippet}"))?
            .extract();

        if meta.title.is_none() || meta.title == Some(String::new()) {
            meta.title = Some(link.to_string());
        }
        if meta.text.is_none() || meta.text == Some(String::new()) {
            meta.text = Some(snippet);
        }
        meta.destination = link.to_string();
        meta.is_code = true;
        Ok(())
    }
}
