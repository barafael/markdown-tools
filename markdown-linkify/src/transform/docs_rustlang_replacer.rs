use anyhow::Context;
use pulldown_cmark::Event;
use regex::Regex;
use tempfile::TempDir;

use crate::{link::Link, LinkTransformer};

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
        Regex::new(r"rust:(?<i>.+)").expect("Invalid regex")
    }

    fn apply(&self, link: &mut Link) -> anyhow::Result<()> {
        // Extract item name
        let snippet = self
            .pattern()
            .replacen(&link.destination, 1, "$i")
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
        let (_full, [url]) = regex
            .captures(&html)
            .with_context(|| format!("No captures found for {snippet}"))?
            .extract();

        link.destination = url.to_string().into();
        if link.title.is_empty() {
            link.title = url.to_string().into();
        }
        if link.text.is_empty() {
            link.text = vec![Event::Text(snippet.into())];
        }
        Ok(())
    }
}
