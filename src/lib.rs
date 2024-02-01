use mdbook::book::Book;
use mdbook::book::{BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use once_cell::sync::Lazy;
use regex::Regex;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

/// The GitBook preprocessor.
pub struct GitBook;

impl Default for GitBook {
    fn default() -> Self {
        Self::new()
    }
}

impl GitBook {
    pub fn new() -> GitBook {
        GitBook
    }
}

impl Preprocessor for GitBook {
    fn name(&self) -> &str {
        "gitbook"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut error: Option<Error> = None;
        book.for_each_mut(|item: &mut BookItem| {
            if error.is_some() {
                return;
            }
            if let BookItem::Chapter(ref mut chapter) = *item {
                if let Err(err) = handle_chapter(chapter) {
                    error = Some(err)
                }
            }
        });
        error.map_or(Ok(book), Err)
    }

    /// Check whether we support the specified renderer
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

/// Apply to all chapters
fn handle_chapter(chapter: &mut Chapter) -> Result<(), Error> {
    chapter.content = inject_stylesheet(&chapter.content)?;
    chapter.content = render_hints(&chapter.content)?;
    chapter.content = render_youtube_embeds(&chapter.content)?;
    Ok(())
}

/// Adds our stylesheet to the chapter
fn inject_stylesheet(content: &str) -> Result<String, Error> {
    let style = Asset::get("style.css").expect("style.css not found in assets");
    let style = std::str::from_utf8(style.data.as_ref())?;
    Ok(format!("<style>\n{style}\n</style>\n{content}"))
}

/// Uses regex to find [GitBook hints](https://docs.gitbook.com/content-editor/blocks/hint)
/// and replaces them with appropriate HTML rendering
fn render_hints(content: &str) -> Result<String, Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r#"\{% hint style="(?P<kind>[^"]+)" %\}\s*\n(?P<body>(?:.*\n)*?)\s*\{% endhint %\}"#,
        )
        .expect("failed to parse regex")
    });
    let hints = Asset::get("hints-template.html").expect("hints-template.html not found in assets");
    let hints = std::str::from_utf8(hints.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let kind = caps
            .name("kind")
            .expect("kind not found in regex")
            .as_str()
            .to_lowercase();
        let body = caps
            .name("body")
            .expect("body not found in regex")
            .as_str()
            .replace("\n>\n", "\n\n")
            .replace("\n> ", "\n");
        hints.replace("{kind}", &kind).replace("{body}", &body)
    });
    Ok(content.into())
}

/// Uses regex to find [GitBook YouTube embeds](https://docs.gitbook.com/content-editor/blocks/embed-a-url#videos)
/// and replaces them with appropriate HTML rendering
fn render_youtube_embeds(content: &str) -> Result<String, Error> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"\{%\s*embed\s*url="https://www\.youtube\.com/watch\?v=(?P<ytid>[a-zA-Z0-9_-]+)"\s*%\}"#)
            .expect("failed to parse regex")
    });
    let embeds_template = Asset::get("youtube-embed-template.html")
        .expect("youtube-embed-template.html not found in assets");
    let hints = std::str::from_utf8(embeds_template.data.as_ref())?;
    let content = RE.replace_all(content, |caps: &regex::Captures| {
        let yt_id = caps
            .name("ytid")
            .expect("YouTube-ID not found in regex")
            .as_str();
        hints.replace("{ytid}", &yt_id)
    });
    Ok(content.into())
}
