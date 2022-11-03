use std::{
    collections::HashSet,
    env::set_current_dir,
    fs,
    path::{Path, PathBuf},
};

use super::*;
use crate::parse::tests::INIT;
use crate::parse::transform_md;

/// Set the current directory to the `src` directory of the test book.
fn set_src_current_dir() {
    INIT.call_once(|| {
        set_current_dir("assets/tests/parse_test/src")
            .expect("Error setting the current directory");
    });
}

/// Return a `Some()` containing the `README.md` test file.
fn get_readme_source_path() -> Option<PathBuf> {
    let readme_path = Path::new("Introduction/README.md").to_path_buf();
    Some(readme_path)
}

/// Return a `Some()` containing the `Subsection.md` test file.
fn get_subsection_source_path() -> Option<PathBuf> {
    let path = Path::new("Introduction/Subsection.md").to_path_buf();
    Some(path)
}

/// Return all paths from `src` folder used in tests.
fn get_book_paths() -> Box<HashSet<PathBuf>> {
    let mut set = HashSet::new();
    set.insert(
        PathBuf::from("Introduction/README.md")
            .canonicalize()
            .unwrap(),
    );
    set.insert(
        PathBuf::from("Introduction/Subsection.md")
            .canonicalize()
            .unwrap(),
    );

    Box::new(set)
}

/// Return a transformed test text.
fn get_transformed_readme() -> String {
    let source_path = get_readme_source_path();
    let mut section_number = vec![0];
    transform_md(
        &fs::read_to_string(source_path.clone().unwrap()).unwrap(),
        0,
        &source_path,
        &get_book_paths(),
        &mut section_number,
    )
}

/// Return a transformed book concatenating README and Subsection.
fn get_transformed_book() -> String {
    let mut book = get_transformed_readme();
    let source_path = get_subsection_source_path();
    let mut section_number = vec![1, 1];
    book.push_str(&transform_md(
        &fs::read_to_string(source_path.clone().unwrap()).unwrap(),
        1,
        &source_path,
        &get_book_paths(),
        &mut section_number,
    ));

    book
}

#[test]
fn test_translate_relative_paths() {
    set_src_current_dir();
    let source_path = get_readme_source_path();
    let mut relative_link = "[My link](Subsection.md)";
    assert_eq!(
        "[My link](Introduction/Subsection.md#)",
        translate_relative_paths(relative_link, &source_path, &get_book_paths())
    );
    relative_link = "[My link](./Subsection.md)";
    assert_eq!(
        "[My link](Introduction/./Subsection.md#)",
        translate_relative_paths(relative_link, &source_path, &get_book_paths())
    );
    relative_link = "[My link](../Introduction/Subsection.md)";
    assert_eq!(
        "[My link](Introduction/../Introduction/Subsection.md#)",
        translate_relative_paths(relative_link, &source_path, &get_book_paths())
    );
    relative_link = "[My link](../../src/Introduction/Level4Section.md)";
    assert_eq!(
        "[My link](Introduction/../../src/Introduction/Level4Section.md)",
        translate_relative_paths(relative_link, &source_path, &get_book_paths())
    );
    let mut reference = "[My link]: Subsection.md";
    assert_eq!(
        "[My link]: Introduction/Subsection.md#",
        translate_relative_paths(reference, &source_path, &get_book_paths())
    );
    reference = "[My link]: ../Introduction/Subsection.md";
    assert_eq!(
        "[My link]: Introduction/../Introduction/Subsection.md#",
        translate_relative_paths(reference, &source_path, &get_book_paths())
    );
}

#[test]
fn test_translate_internal_reference() {
    set_src_current_dir();
    let fixed = fix_internal_links(&get_transformed_readme());
    // assert that the fixed link is in the fixed text
    let pos = fixed.find("(#h1__1__things");
    assert!(pos.unwrap_or(0) > 0);
}

#[test]
fn test_translate_reference_another_file() {
    set_src_current_dir();
    let fixed = fix_external_links(&get_transformed_book());
    // assert that the fixed link is in the fixed text
    let pos = fixed.find("(#h1__2__1__subsection-of-the-subsection)");
    assert!(pos.unwrap_or(0) > 0);
    let pos = fixed.find("(#h1__2__subsection)");
    assert!(pos.unwrap_or(0) > 0);
}

#[test]
fn test_find_header_id_in_text() {
    set_src_current_dir();
    let text = get_transformed_readme();
    assert_eq!(
        "#h1__chapter-1",
        find_header_id_in_text("chapter-1", &text, false).unwrap()
    );
    assert_eq!(
        "#h1__1__things",
        find_header_id_in_text("things", &text, false).unwrap()
    );
    assert_eq!(
        "#h1__chapter-1",
        find_header_id_in_text("things", &text, true).unwrap()
    );
}
