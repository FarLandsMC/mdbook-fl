use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use semver::{Version, VersionReq};
use std::io;
use std::process;

mod replacers;

fn main() {
    let mut args = std::env::args();

    args.next();

    if args.next().is_some() {
        process::exit(0); // We support everything
    } else if let Err(e) = FLPreprocessor.handle_preprocessing() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

pub struct FLPreprocessor;

impl FLPreprocessor {
    fn handle_preprocessing(&self) -> Result<(), Error> {
        let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

        let book_version = Version::parse(&ctx.mdbook_version)?;
        let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

        if !version_req.matches(&book_version) {
            eprintln!(
                "Warning: The {} plugin was built against version {} of mdbook, but we're being called from version {}",
                self.name(),
                mdbook::MDBOOK_VERSION,
                ctx.mdbook_version
            );
        }

        let processed_book = self.run(&ctx, book)?;
        serde_json::to_writer(io::stdout(), &processed_book)?;

        Ok(())
    }
}

impl Preprocessor for FLPreprocessor {
    fn name(&self) -> &str {
        "fl-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|bi| match bi {
            mdbook::BookItem::Chapter(c) => c.content = replacers::handle_content(&c.content),
            mdbook::BookItem::Separator => {}
            mdbook::BookItem::PartTitle(_) => {}
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
