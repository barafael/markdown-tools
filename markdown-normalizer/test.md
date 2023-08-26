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

````rust
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

<iframe width="100%" height="80%" src="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+core%3A%3Aiter%3A%3AFilter%3B%0Ause+std%3A%3Aslice%3A%3AIter%3B%0A%0Afn+main%28%29+%7B%0A++++let+a+%3D+%5B0i32%2C+-1%2C+2%2C+-3%2C+4%5D%3B%0A++++let+mut+iter%3A+Filter%3CIter%3C%27_%2C+i32%3E%2C+_%3E+%3D+a.iter%28%29.filter%28%7Cx%7C+x.is_positive%28%29%29%3B%0A++++%0A++++while+let+Some%28n%29+%3D+iter.next%28%29+%7B%0A++++++++dbg%21%28n%29%3B%0A++++%7D%0A%7D%0A">
</iframe>

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



````rust no_run, marker:test1
link+local
````

---
# Peaches

````marker:peaches
replace
````

---

# Org Mode

```orgmode
begin_block src rust marker:test1
replace+link
end_block
```

---

# How to hack a run button into the html

```html
<button style="
    height: fit-content;
    margin: 5px;
    font-weight: bold;
">Run</button>
```

```html
<div data-marp-auto-scaling-wrapper="" style="
    display: flex;
"><svg part="svg" data-marp-auto-scaling-svg="" viewBox="0 0 1140 190" preserveAspectRatio="xMinYMid meet" style=""><foreignObject width="1140" height="190"><span data-marp-auto-scaling-container="" style="margin-left: 0px; margin-right: auto;"><slot></slot></span></foreignObject></svg><button style="
    height: fit-content;
    margin: 5px;
    font-weight: bold;
">Run</button></div>
```