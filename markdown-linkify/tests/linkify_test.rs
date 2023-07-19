use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;

#[test]
fn linkifies() -> anyhow::Result<()> {
    let input = assert_fs::NamedTempFile::new("in.md")?;
    input.write_str(include_str!("input1.md"))?;

    let output = assert_fs::NamedTempFile::new("out.md")?;

    let mut cmd = Command::cargo_bin("markdown-linkify")?;
    cmd.arg(input.path()).arg("--output").arg(output.path());
    cmd.assert().success();

    assert_eq!(std::fs::read_to_string(output)?, include_str!("output1.md"));
    Ok(())
}
