use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

/// A no-op preprocessor.
pub struct Nop;

impl Nop {
    pub fn new() -> Nop {
        Nop
    }
}

impl Preprocessor for Nop {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        // we *are* a no-op preprocessor after all
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
