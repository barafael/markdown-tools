---

title: How the Codeblock Processor works
marp: true
theme: rhea
color: "dark-gray"

---

## Annotating Snippets

Just annotate your source code with:

````rust marker:meta

````
after running the codeblock processor:

````rust marker:snippet

````

Fun fact:
the two snippets above are inserted using two nested markers.

---

## Opening playground

````rust tag:playground-button playground-before:$"fn main() {"$ playground-after:$"}"$
println!("I am in main");
return anyhow::Ok(())
````

Generated via this block:

``````md marker:plsnippet
```rust tag:playground-button playground-before:$"fn main() {"$ playground-after:$"}"$
println!("I am in main");
return anyhow::Ok(())
```
``````

The processor will run your code through `Rustfmt`.

---

## Playground Snippets

````rust tag:playground-button playground-wrap:main_anyhow
println!("I am in main");
````

Generated via this block:

``````md marker:plsnippet
```rust tag:playground-button playground-wrap:main_anyhow
println!("I am in main");
```
``````

Supported wrappers include `main`, `main_anyhow`, and `main_tokio_anyhow`

---

## Linking to [`doc.rust-lang.org`](http://doc.rust-lang.org)

````md
So many cool types. Like [rust:std::marker::PhantomData].
````

after running markdown-linkify:

So many cool types. Like [rust:std::marker::PhantomData].

---

## Linking to [docs.rs](https://docs.rs)

````md
[](docsrs:https://docs.rs/kord/0.6.1/klib/core/chord/struct.Chord.html)
````

after running markdown-linkify:

[](docsrs:https://docs.rs/kord/0.6.1/klib/core/chord/struct.Chord.html)

---

## Custom Regex-based replacements

With a config such as:

````toml marker:linkifyjira

````

for example, in `linkify.toml`

---

## Custom Regex-based replacements

````md
[PS-128]
````

becomes:

[PS-128]

---

## Custom Regex-based replacements

With a config such as:

````toml marker:linkifykeyword

````

for example, in `linkify.toml`

---

## Custom Regex-based replacements

````md
[keyword:const]
````

becomes:

[keyword:const]

---

## How to generate this demo

Just run

````bash
just all
````

---

## Generating manually: Linkify

Autolink:

````bash marker:autolink

````

---

## Generating manually: Snippets

Extract from source files:
````bash marker:extractsnippets

````

Process Code Blocks:
````bash marker:codeblocks

````

---

## Run MARP

````bash marker:runmarp

````

This will generate a file `final.html`

---
