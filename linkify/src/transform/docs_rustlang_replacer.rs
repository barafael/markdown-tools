use anyhow::Context;
use pulldown_cmark::Event;
use regex::Regex;
use tempfile::TempDir;

use crate::{link::Link, LinkTransformer};

#[derive(Debug, Clone, Default)]
pub struct DocsRustlang;

impl DocsRustlang {
    pub fn new() -> Self {
        Self
    }
}

impl LinkTransformer for DocsRustlang {
    fn tag(&self) -> String {
        String::from("rust:")
    }

    /// Generate a barebones rust project with our input text in a doc comment,
    /// run rustdoc on it,
    /// parse out the result link from the generated html.
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
            .arg("-Z")
            .arg("unstable-options")
            .arg("--extern-html-root-url")
            .arg("core=https://doc.rust-lang.org/stable/")
            .arg("--extern-html-root-url")
            .arg("alloc=https://doc.rust-lang.org/stable/")
            .arg("--extern-html-root-url")
            .arg("std=https://doc.rust-lang.org/stable/")
            .arg("--extern-html-root-takes-precedence")
            .arg("--out-dir")
            .arg(tmp_dir.path())
            .arg(test_file_path)
            .spawn()
            .expect("Failed to spawn rustdoc")
            .wait()
            .expect("Failed awaiting rustdoc result");

        if !output.success() {
            eprintln!("Warning: Rustdoc exited with error {output:?}");
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
            link.text = vec![Event::Code(snippet.into())];
        }
        Ok(())
    }
}
