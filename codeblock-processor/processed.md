---

title: Details über Rust Collections
marp: true
theme: rhea
color: "dark-gray"

---

<!-- 
footer: "Details über Rust Collections"
 -->
<!--
paginate: true
 -->
<!-- 
_footer: ''
_paginate: false
 -->
<!-- _class: lead -->

# Details über Rust Collections

<br>

### It's Iterators all the way down!

---

# Something

<div style="position: relative;">

````rust marker:main
fn main() -> anyhow::Result<()> {
    // Read the Markdown file from disk
    let input = fs::read_to_string("example.md").unwrap();

    // Parse the Markdown input into events
    let parser = Parser::new(&input);

    // Iterate over the events and process code blocks
    let mut in_code_block = false;
````

<p style="position: absolute; right: 10px; top: 10px; padding: 0; margin: 0; line-height: 0">
<button
    onclick="window.location.href='vscode://file/home/rafael/markdown-snippet-inserter/snippet-extractor/tests/rust_files/more/main.rs:12:1'"
    style="
    height: fit-content;
    margin: 0;
    font-weight: bold;"
>OPEN VSCODE
</button>
</p>
</div>

---