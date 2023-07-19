use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn linkifies() -> anyhow::Result<()> {
    let output = assert_fs::NamedTempFile::new("out.md")?;

    let mut cmd = Command::cargo_bin("markdown-linkify")?;
    cmd.arg("tests/input1.md")
        .arg("--output")
        .arg(output.path());
    cmd.assert().success();

    assert_eq!(std::fs::read_to_string(output)?, include_str!("output1.md"));
    Ok(())
}
