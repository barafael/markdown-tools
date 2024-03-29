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

````rust tag:playground-button
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
````

Ziemlich viele Rust Konzepte in diesem Snippet!

* Assoziierter Typ `Item`: erlaubt es, den Element-Typ festzulegen
* Methode `next`: Nächstes Element, oder `None`

---

# It's a playground

<iframe width="100%" height="80%" tabindex="-1" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+core%3A%3Aiter%3A%3AFilter%3B%0Ause+std%3A%3Aslice%3A%3AIter%3B%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%5B0i32%2C+-1%2C+2%2C+-3%2C+4%5D%3B%0A++++let+mut+iter%3A+Filter%3CIter%3C%27_%2C+i32%3E%2C+_%3E+%3D+a.iter%28%29.filter%28%7Cx%7C+x.is_positive%28%29%29%3B%0A++++%0A++++while+let+Some%28n%29+%3D+iter.next%28%29+%7B%0A++++++++dbg%21%28n%29%3B%0A++++%7D%0A%7D%0A">
</iframe>

---

# A main method

````rust tag:playground-button playground-wrap:main_anyhow
    // Read the Markdown file from disk
    let input = fs::read_to_string("example.md")?;

    // Parse the Markdown input into events
    let parser = Parser::new(&input);

    // Iterate over the events and process code blocks
    let mut in_code_block = false;
````

---
