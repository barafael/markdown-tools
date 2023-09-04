use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[ignore = "Does not pass on github CI, either because rustdoc is missing or temporary directories are not allowed there"]
fn linkifies() -> anyhow::Result<()> {
    let output = assert_fs::NamedTempFile::new("out.md")?;

    let mut cmd = Command::cargo_bin("markdown-linkify")?;
    cmd.arg("tests/input1.md")
        .arg("--output")
        .arg(output.path());
    cmd.assert().success();

    let doc = std::fs::read_to_string(output.path()).unwrap();
    //println!("{}", &doc);
    assert_eq!(doc, include_str!("../tests/output1.md"));
    Ok(())
}
