# Project README

A short description of the project with **bold** and *italic* text.

## Features

- [x] Markdown parsing
- [x] GFM support
- [ ] Live preview
- [ ] Syntax highlighting

## Installation

```bash
cargo install birta
```

## Usage

| Command | Description |
|---------|-------------|
| `birta file.md` | Preview a file |
| `birta --port 3000 file.md` | Use specific port |

## API

The main function accepts a `&str` and returns rendered HTML:

```rust
let html = birta::render("# Hello");
assert!(html.contains("<h1>"));
```

## Notes

This project uses comrak[^1] for parsing.

> Blockquotes are supported too.
>
> Even with multiple paragraphs.

---

Check out https://example.com for more information.

[^1]: Comrak is a GFM-compatible markdown parser written in Rust.
