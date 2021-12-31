use std::{fs::File, io::Read, path::PathBuf};

use mdbook::{book::Book, renderer::RenderContext, Config};

use crate::{config::TitleLabels, parse::parse_book};

#[test]
fn test_parse_book() {
    let correct_parse = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/tests/parse_test/book.md"
    ));
    let title_labels = TitleLabels {
        preamble: String::from("Preamble"),
        chapters: String::from("Chapters"),
        annexes: String::from("Annexes"),
    };
    let ctx = RenderContext::new(
        PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/tests/parse_test"
        )),
        // the book itself is not used by the `parse_book` function so never mind
        Book::new(),
        // config is also not used
        Config::default(),
        PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/tests/parse_test/book/pandoc"
        )),
    );
    let result_path = parse_book(&ctx, &title_labels);
    let mut result_bytes = Vec::new();
    File::open(result_path)
        .expect("Error opening the parsed file")
        .read_to_end(&mut result_bytes)
        .expect("Error reading the parsed file");
    assert_eq!(correct_parse.to_vec(), result_bytes);
}
