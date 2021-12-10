//! mdBook backend that is able to generate many output formats using `pandoc`.
//! Note that `pandoc` must be installed in your system. You also need other
//! tools for some formats like PDF.
//!
//! The following opinionated rules are applied to the book, supposing that
//! headers that are not the main header are used only to highlight and
//! separate elements in the page but they must not be in the table of contents
//! nor be numbered (if an actual title is desired its contents should be in a
//! different Markdown file):
//! - If the book have prefixes or suffixes a part is created for each of them.
//! - If parts are created for prefixes and/or suffixes and the numbered
//! chapters don't have parts then a part is created for the chapters, the
//! original parts are used for the numbered chapters otherwise.
//! - The headers of the files are downgraded adding the level of the file
//! (e.g. a chapter inside a part has level 1 so its 1-level header becomes 2-level).
//! - If a header's level becomes bigger than 6 then the text is simply bold.
//! - Setext headers (underlined ones) are changed to atx (hashes prefix).
//! - Headers that are not the main one are labeled with `{.unnumbered .unlisted}`
//! to remove numbers and avoid that they appear in the table of contents.
//! - Each Markdown file must have a main 1st level header as the first header
//! of the file and that header is used as its header, the name of the file is
//! ignored.

mod parse;

use mdbook::renderer::RenderContext;
use parse::parse_book;
use std::io;

/// The main function that parses the book and generates outputs.
fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    let preamble_text = "Preamble";
    let chapters_text = "Chapters";
    let annexes_text = "Annexes";

    parse_book(&ctx, preamble_text, chapters_text, annexes_text);
}
