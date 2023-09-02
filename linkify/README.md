# Markdown Linkify

Replace link shorthands, such as

```md
In [PS-128], the issue is described in detail.
```

with actual links. For example, the above could be transformed to:

```md
In [PS-128](https://www.company.jira.com/issues/PS-128), the issue is described in detail.
```

Custom replacers exist, so far only for [docs.rs](https://www.docs.rs) and [doc.rust-lang.org/](https://doc.rust-lang.org/stable/).

See `test.md` for the supported expressions.

## Supported link types

Supports `[tag]`-style "broken links" and many forms of `[title or empty](tag "optional hover text")`.

The specific replacements depend on the matching transformer.

## TODO

* [x] code blocks in links
* [x] make regex replacer which can be configured by its own file
* [x] make regex replacer config files for the existing usecases
* [ ] org-mode frontend for agnostic replacers
* [ ] local rustdoc replacer
* [ ] collect replacer ideas, github issue replacer?
* [x] unit tests for replacers, linkify
* [ ] documentation
* [x] support composite text events
* [x] implement LinkMetadata as iterator
* [x] figure out how to publish linkify on crates.io
* [ ] check how code insertion should actually work