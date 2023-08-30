---
title: "Consequences of Rigorosity"
description: Some good, some __interesting__
marp: true
theme: rhea
color: "dark-gray"
size: 16:9

---

<!--
paginate: true
 -->

<!-- 
_footer: ''
_paginate: false
 -->

<!-- _class: lead -->

# Consequences of Rigorosity

## Some good, some _interesting_

---

## Syntax Consistencies

Rigorosity leads to a number of wild consequences in syntax, impeding approachability.

Operator precedence:

````rust tag:playground-button playground-before:$"fn main() {\n"$ playground-after:$"\n}"$
dbg!(-9_i32.abs());
````

What will this code print, and why?

<!-- footer: '"WTF is this, I have never seen anything like it" - moments'-->

---

## Rust Code in VSCode

````rust marker:main

````

---

## Syntax Consistencies

"Variables" aren't "assigned", instead "values" are "bound".
Values may be re-bound at any point, which will drop the previously bound value.

Re-binding (a.k.a shadowing):

````rust tag:playground-button playground-before:$"fn main() -> anyhow::Result<()> {"$ playground-after:$"dbg!(age);Ok(())}"$
let age = "7"; // A string literal is of type &str
let age = age.parse(); // parse takes a &self, returns a Result<T, ParseError>
let age: u32 = age.unwrap(); // unwrap takes self
````

`age` is borrowed and moved here. Can you spot where?

---

## Syntax Consistencies

Rigorosity leads to a number of wild consequences in syntax, impeding approachability:

Expression orientation is weird at first:

````rust
expression orientation is weird at first
````

---

## Typographical Upsets

Macros look funny/unusual at times:
````rust
let item = events.next().await?;
assert!(!matches(item, Event::Heartbeat))
````

---