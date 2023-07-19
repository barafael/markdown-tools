use assert_cmd::prelude::*;
use snippet::Snippets;
use std::{fs::File, process::Command};

#[test]
fn makes_snippets() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("snippet-extractor")?;
    let output = assert_fs::NamedTempFile::new("out.json")?;

    cmd.arg("--directory")
        .arg("tests/rust_files")
        .arg("--output")
        .arg(output.path());
    cmd.assert().success();

    let snippets: Snippets = serde_json::from_reader(File::open(output.path())?)?;
    let expected: Snippets = serde_json::from_str(include_str!("test1.json"))?;

    assert_eq!(snippets, expected);
    Ok(())
}
