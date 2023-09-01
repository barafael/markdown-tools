---

title: How the Codeblock Processor works
marp: true
theme: rhea
color: "dark-gray"

---

## Annotating Snippets

Just annotate your source code with:

````rust
// marker-start meta
fn some_code() {}
// marker-end meta
````
after running the codeblock processor:

````rust marker:meta

````

Note: when nested markers are supported, the first snippet should be inserted via codeblock processor, too.

---

## Opening playground

````rust tag:playground-button playground-before:$"fn main() {"$ playground-after:$"}"$
println!("I am in main");
return anyhow::Ok(())
````

The processor will run your code through `Rustfmt`.

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

````toml
[[regex]]
tag = 'PS-'
tail = '\d+'
replacement = "https://internal.jira.com/$text"
limit = 1
strip_tag = false
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

## How to generate this demo

Just run

````bash
just all
````

---

## Generating manually: Linkify

Autolink:
````bash
markdown-linkify blob.md --config linkify.toml --output processed.md
````

---

## Generating manually: Snippets

Extract from source files:
````bash
snippet-extractor --directory . --output snippets.json
````

Process Code Blocks:
````bash
markdown-codeblock-processor processed.md --snippets snippets.json -o final.md
````

---

## Run MARP

````bash
marp --allow-local-files --html --pdf-outlines.headings true
--pdf-outlines.pages true final.md --html true --theme ../rhea.css
````

This will generate a file `final.html`

---
