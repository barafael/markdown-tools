# markdown-tools

A fun collection of different tools which transform markdown to different markdown in some or other useful way.

## TODO

[ ] make a justfile for sequential application of all the tools?

[x] make snippet-extractor optionally emit relative paths

[ ] remove file content from snippets?

[ ] support nested snippets, perhaps even overlapping snippets

[ ] snippet-compiler: extract snippets, run them through rustc+clippy

[ ] make linkify.toml optional

[ ] support `//` and `#` in marker-start, marker-end, such that the linkify.toml in the demo folder can be also included as snippet, as well as markers inside the `justfile`

[ ] Make sure the linkify tool works with all replacers on all link types

[ ] Reduce the number of unwraps

[ ] Consider implementing a codeblock iterator like done with the linkify tool

[ ] Make snippet-extractor work with `.` as argument, and make it ignore non-utf-8 files, so that it will pick up snippets from linkify.toml and justfile

[ ] Make snippet-extractor respect .gitignore
