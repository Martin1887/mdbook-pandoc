use std::{
    fs::{remove_dir_all, File},
    io::Read,
    path::PathBuf,
    sync::Once,
};

use env_logger::Env;
use log::warn;
use mdbook::MDBook;

use crate::{config::MdBookPandocConfig, process_metadata_block, PandocRenderer};

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

/// Test the full Pandoc transformation of the book checking that the output MD
/// file is equal to the expected one and that the transformed files are
/// generated.
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

    // Clean the output directory.
    let output_dir_path = root_path.join("book/pandoc");
    remove_dir_all(output_dir_path.clone()).expect("Error cleaning the output directory");

    let result_path = output_dir_path.clone().join("book.md");
    let pdf_path = output_dir_path.clone().join("book.latex.pdf");
    let epub_path = output_dir_path.join("book.epub");

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

    assert!(pdf_path.is_file());
    assert!(epub_path.is_file());
}

/// Test that the metadata block is correctly placed only if it has contents.
#[test]
fn test_process_metadata_block() {
    let contents = r#"
---
author: Test author
...

Test contents.
"#
    .to_string();
    let no_metadata_config: MdBookPandocConfig = toml::from_str(
        r#"
[general]    
"#,
    )
    .unwrap();
    let empty_metadata_config: MdBookPandocConfig = toml::from_str(
        r#"
[general]

[general.epub_metadata_fields]    
"#,
    )
    .unwrap();
    let with_metadata_config: MdBookPandocConfig = toml::from_str(
        r#"
[general]

[general.epub_metadata_fields]
title = "Test title"
"#,
    )
    .unwrap();

    assert_eq!(
        contents,
        process_metadata_block(
            contents.clone(),
            &no_metadata_config.general.epub_metadata_fields
        )
    );
    assert_eq!(
        contents,
        process_metadata_block(
            contents.clone(),
            &empty_metadata_config.general.epub_metadata_fields
        )
    );
    assert_eq!(
        format!("---\ntitle: Test title\n---\n\n{}", contents),
        process_metadata_block(contents, &with_metadata_config.general.epub_metadata_fields)
    );
}
