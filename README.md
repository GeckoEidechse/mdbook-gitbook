# mdbook-gitbook

[![crates.io](https://img.shields.io/crates/v/mdbook-gitbook.svg)](https://crates.io/crates/mdbook-gitbook)
[![MPL 2.0 LICENSE](https://img.shields.io/github/license/rust-lang/mdBook.svg)](LICENSE)
[![docs.rs](https://docs.rs/mdbook-gitbook/badge.svg)](https://docs.rs/mdbook-gitbook)
[![Build](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/build.yml/badge.svg)](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/build.yml)
[![Test](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/test.yml/badge.svg)](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/test.yml)

[mdBook](https://github.com/rust-lang/mdBook) preprocessor to properly render [GitBook](https://www.gitbook.com/) specific syntax in mdBook.

You can compare this preprocessor to GitBook by looking at the same sample book on:

- [GitBook](https://mdbook.gitbook.io/mdbook-gitbook/)
- [mdBook](https://geckoeidechse.github.io/mdbook-gitbook/) (via GitHub pages)

To see the list of existing and supported syntax, see [this GitHub issue](https://github.com/GeckoEidechse/mdbook-gitbook/issues/1).

> [!WARNING]
>
> This pre-processor is still heavily work-in-progress.
> Contributions of all kinds, especially to extend the supported GitBook syntax, are heavily encouraged and welcome <3

## Usage

First, install the preprocessor:

```bash
cargo install mdbook-gitbook
```

Then, add the preprocessor to your `book.toml`:

```toml
[book]
authors = ["Jill Doe"]
language = "en"
multilingual = false
src = "src"
title = "My awesome Book"

# ADD THIS
[preprocessor.gitbook]

```
