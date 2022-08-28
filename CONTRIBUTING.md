# Lyrics file syntax

Text in lyrics files is mostly presented on the website as-is, including
leading whitespace.

## Vocalist annotations

You can begin a line with, for example, `Gishnchips:: ` to indicate that it was
performed by Gishnchips. The parser keeps a record of the last-mentioned
performer, so you only need to annotate performer when it changes.

Performer annotations can only exist at the beginning of lines in the text
file. If you want to present a change of performer in the middle of a line in
the output, you can do so by putting some whitespace before an annotation:

```
Gishnchips:: This is a line performed by Gish
This is another line performed by Gish
Mos Prob:: This one's by Mos Prob
And this one
  Gishnchips:: changes who is speaking
  Both:: a few times
```

### Aliases

Typing the full names of people or characters who appear regularly would be
tedious, so you can use aliases. `M::` expands to `Mos Prob::`, for example.
The full list is defined as `ALIASES` in [lyrics.rs](./src/lyrics.rs)

### Leading whitespace

Other than the first space, whitespace after an annotation (or at the beginning
of a line without any annotation) is retained in the output, so if you want to
indent some text and change performer on the same line, be sure to put the
indentation after the annotation.

# Running locally

If you want to test your changes locally, here's what you do:

- [install rust](https://www.rust-lang.org/tools/install)
- clone this repository
- run `cargo run` from the root of the repository
- open `index.html` in the `_html` folder that was created
