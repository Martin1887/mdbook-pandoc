use std::{fs::File, io::Read, path::PathBuf, sync::Once};

use env_logger::Env;
use log::warn;
use mdbook::MDBook;

use crate::PandocRenderer;

static INIT: Once = Once::new();

/// Initialize the log in test mode.
fn setup() {
    INIT.call_once(|| {
        let log_result = env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
            .is_test(true)
            .try_init();
        if log_result.is_err() {
            warn!("Error initializing the log");
        }
    });
}

#[test]
fn test_parse_book() {
    setup();
    let correct_parse = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/tests/test_book/correct_book.md"
    ));

    let root_path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/tests/test_book"
    ));

    let result_path = root_path.join("book/pandoc/book.md");

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
