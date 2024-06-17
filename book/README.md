## Books
This directory contains the source files for `mw`'s user book.

The source files are edited here, and published from this repository.

## Build tools
Building the book requires [Rust's cargo tool](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [mdBook](https://github.com/rust-lang/mdBook).

After installing `cargo`, install `mdbook` with:
```bash
cargo install mdbook
```

## Building
To build a book, go into this directory and build:

```bash
cd book/
mdbook build
```

The output will be in the `book` subdirectory. To open the book, you can open it in your web browser like so:
```bash
mdbook build --open
```
