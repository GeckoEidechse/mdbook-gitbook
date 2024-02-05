# mdbook-gitbook

[![crates.io](https://img.shields.io/crates/v/mdbook-gitbook.svg)](https://crates.io/crates/mdbook-gitbook)
[![MPL 2.0 LICENSE](https://img.shields.io/github/license/GeckoEidechse/mdbook-gitbook.svg)](LICENSE)
[![docs.rs](https://docs.rs/mdbook-gitbook/badge.svg)](https://docs.rs/mdbook-gitbook)
[![Build](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/build.yml/badge.svg)](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/build.yml)
[![Test](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/test.yml/badge.svg)](https://github.com/GeckoEidechse/mdbook-gitbook/actions/workflows/test.yml)

[mdBook](https://github.com/rust-lang/mdBook) preprocessor to properly render [GitBook](https://www.gitbook.com/) specific syntax in mdBook.

You can compare this preprocessor to GitBook by looking at the same sample book on:

- [GitBook](https://mdbook.gitbook.io/mdbook-gitbook/)
- [mdBook](https://geckoeidechse.github.io/mdbook-gitbook/) (via GitHub pages)

To see the list of existing and supported syntax, see [this GitHub issue](https://github.com/GeckoEidechse/mdbook-gitbook/issues/1).

> **Note:**
>
> This mdBook preprocessor was written for local rendering of the [NorthstarWiki](https://github.com/R2Northstar/NorthstarWiki/) where it covers the majority of the used GitBook-specific syntax.
> As such I'm not planning to add support for more GitBook widgets etc, however contributions to expand the supported syntax are more than welcome <3
>
> Similarly, if you'd like to take over maintainership or ownership of this crate, please get in touch via an issue in the GitHub repo of this crate.

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

## Development

### How it works

The way this preprocessor works is primarily by using regexes to search for specific patterns like

```
{% embed url="URL_HERE" %}
```

and then replacing it with the corresponding HTML code like

```html
<div style="border: 1px solid #ccc; padding: 10px; max-width: 500px; margin: 10px">
  <a href="URL_HERE">
    <div style="display: flex; align-items: center;">
      <div style="margin-right: 10px;">
        <img alt="" src="{icon_link}" width="100%" height="auto" decoding="async"
          style="width: 32px; height: 32px; border-radius: 4px;">
      </div>
      <div style="flex-grow: 1;">
        <div style="font-weight: bold; margin-bottom: 5px;">
          EXTRACTED_PAGE_TITLE
        </div>
        <div style="color: #666;">
          EXTRACTED_PAGE_NAME
        </div>
      </div>
    </div>
  </a>
</div>
```

### Expanding the preprocessor

The currently supported syntax is tracked in [this GitHub issue](https://github.com/GeckoEidechse/mdbook-gitbook/issues/1)

To add support for some currently unsupported syntax, expand the existing existing main render loop in `lib.rs`

```rust
/// Apply to all chapters
fn handle_chapter(chapter: &mut Chapter) -> Result<(), Error> {
    chapter.content = hints::render(&chapter.content)?;
    chapter.content = embeds::render(&chapter.content)?;
    chapter.content = content_refs::render(&chapter.content)?;
    // Add your additional syntax parsing here

    Ok(())
}
```

with a function that calls the corresponding parsing logic.

In your parsing logic, use regex or any other methods to scan for the specific pattern of the syntax you want to support and replace it with the corresponding HTML code.
