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

# Grundlage: Iteratoren in Rust

````rust no_run, marker:test1
marker marker marker
````

Ziemlich viele Rust Konzepte in diesem Snippet!

* Assoziierter Typ `Item`: erlaubt es, den Element-Typ festzulegen
* Methode `next`: Nächstes Element, oder `None`

---

````rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
````

---

# A main method

````rust marker:main
fn main() -> anyhow::Result<()> {
    // Read the Markdown file from disk
    let input = fs::read_to_string("example.md").unwrap();

    // Parse the Markdown input into events
    let parser = Parser::new(&input);

    // Iterate over the events and process code blocks
    let mut in_code_block = false;
````

---

# Peaches

````marker:peaches
let a = 4;
let a = 5;
````