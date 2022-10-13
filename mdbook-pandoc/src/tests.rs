use std::{fs::File, io::Read, path::PathBuf};

use mdbook::MDBook;

use crate::PandocRenderer;

#[test]
fn test_parse_book() {
    let correct_parse = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/tests/parse_test/correct_book.md"
    ));

    let root_path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/tests/parse_test"
    ));

    let mut result_path = root_path.clone();
    result_path.push("book/pandoc/book.md");

    MDBook::load(root_path.clone())
        .expect("Error loading the book")
        .execute_build_process(&PandocRenderer)
        .expect("Error building the book");

    let mut result_bytes = Vec::new();
    File::open(result_path)
        .expect("Error opening the parsed file")
        .read_to_end(&mut result_bytes)
        .expect("Error reading the parsed file");
    assert_eq!(correct_parse.to_vec(), result_bytes);
}
