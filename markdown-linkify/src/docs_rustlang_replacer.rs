use regex::Regex;
use tempdir::TempDir;

use crate::Replacer;

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
    fn apply(&self, snippet: &str) -> Option<String> {
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

        //loop {
        //std::thread::sleep(Duration::from_secs(1));
        //}

        let html = std::fs::read_to_string(result_file_path).ok()?;
        let regex = Regex::new(r###"(?<l>https://doc.rust-lang.org/[^"]+)""###).unwrap();
        let (_full, [thing]) = regex.captures(html.as_str()).unwrap().extract();

        Some(thing.to_string())
    }
}
