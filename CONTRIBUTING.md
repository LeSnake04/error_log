# Genral Rules

Here are some basic instructions for contributions. Try to follow them, even
when maintainers sometimes may forget to follow them ("Do as I say, not as I
do")

- No unsafe code unless approved by mainatiner and gated by "unsafe" feature
  (Will be added when nessesary)
- run cargo spellcheck before the final review and release

## Style Guidelines

- Docs should be similar to the std docs
- Document all public items and ideally privates ones as well
- Doc links should be wrapped in \`: `[`ErrorLog`]`

## Commit Prefixes

- changes only consisting of doc-comment changes: "doc: "
- mdbook (/docs/) changes: "book: "
- test changes: "test:"
- examples added or changed: "ex: "
- added features "feat: "
- fixes: "fix: "
