---

title: How the Codeblock Processor works
marp: true
theme: rhea
color: "dark-gray"

---

## Annotating Snippets

Just annotate your source code with:

<div style="position: relative;">

````rust
// marker-start: meta
fn some_code() {}
// marker-end: meta
````

</div>

after running the codeblock processor:

<div style="position: relative;">

````rust marker:meta
fn some_code() {}
````

<p style="position: absolute; right: 10px; top: 10px; padding: 0; margin: 0; line-height: 0">
<button
    onclick="window.open('vscode://file/'.concat(make_path('demo/demo.rs:2:1')),'_blank')"
    style="
    height: fit-content;
    margin: 0;
    font-weight: bold;"
>Open VSCode
</button>
</p>
</div>

Note: when nested markers are supported, the first snippet should be inserted via codeblock processor, too.

---

## Opening playground

<div style="position: relative;">

````rust tag:playground-button playground-before:$"fn main() {"$ playground-after:$"}"$
println!("I am in main");
return anyhow::Ok(())
````

<p style="position: absolute; right: 10px; top: 10px; padding: 0; margin: 0; line-height: 0">
<button
    onclick="window.open('https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20println%21%28%22I%20am%20in%20main%22%29%3B%0A%20%20%20%20return%20anyhow%3A%3AOk%28%28%29%29%3B%0A%7D%0A','_blank')"
    style="
    height: fit-content;
    margin: 0;
    font-weight: bold;"
>Playground
</button>
</p>
</div>

The processor will run your code through `Rustfmt`.

---

## Linking to [`doc.rust-lang.org`](http://doc.rust-lang.org)

<div style="position: relative;">

````md
So many cool types. Like [rust:std::marker::PhantomData].
````

</div>

after running markdown-linkify:

So many cool types. Like [std::marker::PhantomData](https://doc.rust-lang.org/stable/core/marker/struct.PhantomData.html "https://doc.rust-lang.org/stable/core/marker/struct.PhantomData.html").

---

## Linking to [docs.rs](https://docs.rs)

<div style="position: relative;">

````md
[](docsrs:https://docs.rs/kord/0.6.1/klib/core/chord/struct.Chord.html)
````

</div>

after running markdown-linkify:

[Chord](https://docs.rs/kord/0.6.1/klib/core/chord/struct.Chord.html "https://docs.rs/kord/0.6.1/klib/core/chord/struct.Chord.html")

---

## Custom Regex-based replacements

With a config such as:

<div style="position: relative;">

````toml
[[regex]]
tag = 'PS-'
tail = '\d+'
replacement = "https://internal.jira.com/$text"
limit = 1
strip_tag = false
````

</div>

for example, in `linkify.toml`

---

## Custom Regex-based replacements

<div style="position: relative;">

````md
[PS-128]
````

</div>

becomes:

[PS-128](https://internal.jira.com/PS-128 "https://internal.jira.com/PS-128")

---

## How to generate this demo

Just run

<div style="position: relative;">

````bash
just all
````

</div>

---

## Generating manually: Run Linkify

Autolink:

<div style="position: relative;">

````bash
markdown-linkify blob.md --config linkify.toml --output processed.md
````

</div>

---

## Generating manually: Snippets:

Extract from source files:

<div style="position: relative;">

````bash
snippet-extractor --directory . --output snippets.json
````

</div>

Process Code Blocks:

<div style="position: relative;">

````bash
markdown-codeblock-processor processed.md --snippets snippets.json -o final.md
````

</div>

---

## Run MARP

<div style="position: relative;">

````bash
marp --allow-local-files --html --pdf-outlines.headings true
--pdf-outlines.pages true final.md --html true --theme ../rhea.css
````

</div>

This will generate a file `final.html`

---

<script>
    function make_path(str) {
        return window.location.pathname.substring(0, window.location.pathname.lastIndexOf("/")).concat("/").concat(str);
    }
</script>