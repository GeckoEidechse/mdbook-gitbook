use mdbook::book::Book;
use mdbook::book::{BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use rust_embed::RustEmbed;

mod embeds;
mod hints;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

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
    chapter.content = hints::render_hints(&chapter.content)?;
    chapter.content = embeds::render_youtube_embeds(&chapter.content)?;
    Ok(())
}

/// Adds our stylesheet to the chapter
fn inject_stylesheet(content: &str) -> Result<String, Error> {
    let style = Asset::get("style.css").expect("style.css not found in assets");
    let style = std::str::from_utf8(style.data.as_ref())?;
    Ok(format!("<style>\n{style}\n</style>\n{content}"))
}
