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

````rust, no_run, marker:test1, context:iterator
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
````

Ziemlich viele Rust Konzepte in diesem Snippet!

* Assoziierter Typ `Item`: erlaubt es, den Element-Typ festzulegen
* Methode `next`: Nächstes Element, oder `None`

---
