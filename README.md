# markdown-tools

A fun collection of different tools which transform markdown to different markdown in some or other useful way.

## TODO

- [x] make a justfile for sequential application of all the tools?

- [x] make snippet-extractor optionally emit relative paths

- [x] remove file content from snippets? NO

- [x] support nested snippets, perhaps even overlapping snippets

- [ ] snippet-compiler: extract snippets, run them through rustc+clippy

- [x] make linkify.toml optional

- [x] Unify all the CLI arguments, everywhere

- [x] support `//` and `#` in marker-start, marker-end, such that the linkify.toml in the demo folder can be also included as snippet, as well as markers inside the `justfile`

- [ ] Make sure the linkify tool works with all replacers on all link types

- [x] Reduce the number of unwraps

- [ ] Consider implementing a codeblock iterator like done with the linkify tool. It should stream (codeblock, button options, header/footer html)

- [x] Make snippet-extractor work with `-d .` as argument, and make it ignore non-utf-8 files, so that it will pick up snippets from linkify.toml and justfile

- [x] Make snippet-extractor respect .gitignore

- [x] lib-ificate the implementations with thin bin+cli+config file frontends

- [x] Move the snippet crate into another crate or rename it to something ominous, in order to be able to publish it. Or, incorporate it into the snippet-extractor public API.

- [ ] Allow suppressing button generation via fence attribute

- [ ] Local file link replacer: Simple replacer that allows opening files (link linkify.toml in the demo.md as an example)

- [ ] wildly improve the demo.md

- [x] support wrapping playground text with predefined wrappers like main, anyhow-main, and tokio-anyhow-main

- [x] make the generated buttons never have focus

- [x] process faulty code anyway in playground inserter (Rustfmt)

- [x] support `playground-indent`

- [x] support channel for playground for example via `playground-channel:nightly`

- [ ] formatter and normalizer type processors should probably have a -i "inline" flag

- [x] marker syntax uses space, whereas annotations in md use `:`. This is not ideal.

- [x] be more lenient when path prefix doesn't strip off current_dir.

- [ ] Probably should remake (relative) path handling?

- [ ] support paths on windows

- [ ] coordinator binary which:
  * generates justfile
  * generates template linkify.toml
  * generates .gitignore template
  * Packages projects and presentations in a zip file

- [x] support suppressing other markers in a snippet while including it via annotation in the source (`hide_other_markers`) -> useful for nested and overlapping markers

- [ ] in rhea.css, footers with links that are `code` do not look good: `<!-- _footer: 'Inspired by [`serde_json::Value`](https://docs.rs/serde_json/1.0.106/serde_json/enum.Value.html)'-->`

- [ ] // marker-pause:x,y,z comments which skip a section, delimited by marker-resume:x,y,z. Probably needs grammar or something to be really clean...

- [ ] support links in footnotes (at least [`thing`](rust:std::thing) does not work)