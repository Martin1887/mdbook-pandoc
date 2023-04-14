use std::{
    collections::HashSet,
    env::set_current_dir,
    fs,
    path::{Path, PathBuf},
};

use env_logger::Env;
use log::warn;
use unicase::UniCase;

use super::*;
use crate::parse::tests::INIT;
use crate::parse::transform_md;

/// Initialize the log in test mode and the current path.
fn setup() {
    INIT.call_once(|| {
        set_current_dir("assets/tests/test_book/src").expect("Error setting the current directory");
        let log_result = env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
            .is_test(true)
            .try_init();
        if log_result.is_err() {
            warn!("Error initializing the log");
        }
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
fn get_book_paths() -> HashSet<PathBuf> {
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

    set
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
        true,
        true,
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
        true,
        true,
    ));

    book
}

#[test]
fn test_translate_relative_paths() {
    setup();
    let source_path = get_readme_source_path();
    let book_paths = get_book_paths();
    let mut relative_link = "[My link](Subsection.md)";
    assert_eq!(
        "[My link](Introduction/Subsection.md#)",
        translate_relative_paths(relative_link, &source_path, &book_paths)
    );
    // a backslash avoids the link
    relative_link = r"\[My link](Subsection.md)";
    assert_eq!(
        relative_link,
        translate_relative_paths(relative_link, &source_path, &book_paths)
    );
    relative_link = "[My link]( ./Subsection.md)";
    assert_eq!(
        "[My link]( Introduction/./Subsection.md#)",
        translate_relative_paths(relative_link, &source_path, &book_paths)
    );
    relative_link = "[My link](< ./Subsection.md>)";
    assert_eq!(
        relative_link,
        translate_relative_paths(relative_link, &source_path, &book_paths),
        "Spaces inside destinations between `<` and `>` belong to the path"
    );
    relative_link = "[My link](\n./Subsection.md)";
    assert_eq!(
        relative_link,
        translate_relative_paths(relative_link, &source_path, &book_paths),
        "Link destinations cannot contain line breaks"
    );
    relative_link = "[My link](../Introduction/Subsection.md \n  'My link')";
    assert_eq!(
        "[My link](Introduction/../Introduction/Subsection.md# \n  'My link')",
        translate_relative_paths(relative_link, &source_path, &book_paths)
    );
    relative_link = "[My\nlink](../Introduction/Subsection.md \n 'My link')";
    assert_eq!(
        relative_link,
        translate_relative_paths(relative_link, &source_path, &book_paths),
        "Link texts cannot contain line breaks"
    );
    relative_link = "[My link](<../../src/Introduction/Level4Section.md>)";
    assert_eq!(
        "[My link](<Introduction/../../src/Introduction/Level4Section.md>)",
        translate_relative_paths(relative_link, &source_path, &book_paths)
    );
    let mut reference = "[My link]: Subsection.md";
    assert_eq!(
        "[My link]: Introduction/Subsection.md#",
        translate_relative_paths(reference, &source_path, &book_paths)
    );
    reference = "[My link]: ../Introduction/Subsection.md";
    assert_eq!(
        "[My link]: Introduction/../Introduction/Subsection.md#",
        translate_relative_paths(reference, &source_path, &book_paths)
    );
    // absolute path
    let mut absolute_path = source_path.clone().unwrap().canonicalize().unwrap();
    let mut absolute_path_str = absolute_path.to_str().unwrap();
    let mut reference = format!("[My link]: {}", absolute_path_str);
    assert_eq!(
        format!("{}#", reference),
        translate_relative_paths(&reference, &source_path, &book_paths)
    );
    // incorrect absolute path, should not modify the link and show a warning
    absolute_path = absolute_path.parent().unwrap().join("imaginary_file");
    absolute_path_str = absolute_path.to_str().unwrap();
    reference = format!("[My link]: {}", absolute_path_str);
    assert_eq!(
        reference,
        translate_relative_paths(&reference, &source_path, &book_paths)
    );

    // URI
    reference = String::from("[My link]: https://rust-lang.org");
    assert_eq!(
        reference,
        translate_relative_paths(&reference, &source_path, &book_paths)
    );
    // URI with section
    reference = String::from("[My link]: https://rust-lang.org#section");
    assert_eq!(
        reference,
        translate_relative_paths(&reference, &source_path, &book_paths)
    );
}

#[test]
fn test_translate_internal_reference() {
    setup();
    let fixed = fix_internal_links(&get_transformed_readme());
    // assert that the fixed link is in the fixed text
    let pos = fixed.find("(#h1__1__things");
    assert!(pos.unwrap_or(0) > 0);
}

#[test]
fn test_translate_reference_another_file() {
    setup();
    let fixed = fix_external_links(&get_transformed_book());
    // assert that the fixed link is in the fixed text
    let pos = fixed.find("#h1__2__1__subsection-of-the-subsection");
    assert!(pos.unwrap_or(0) > 0);
    let pos = fixed.find("(#h1__2__subsection)");
    assert!(pos.unwrap_or(0) > 0);
}

#[test]
fn test_find_heading_id_in_text() {
    setup();
    let text = get_transformed_readme();
    assert_eq!(
        "#main-chapter",
        find_heading_id_in_text("main-chapter", &text, false).unwrap()
    );
    assert_eq!(
        "#h1__1__things",
        find_heading_id_in_text("things", &text, false).unwrap()
    );
    assert_eq!(
        "#main-chapter",
        find_heading_id_in_text("things", &text, true).unwrap(),
        "Requesting the first header, it its not returned"
    );
}

#[test]
fn test_normalize_label() {
    let mut label = "ẞ";
    assert_eq!(UniCase::new("SS"), normalize_label(label));
    label = "    Things with \n multiple  spaces and needed to trim\t.\n";
    assert_eq!(
        UniCase::new("Things with multiple spaces and needed to trim ."),
        normalize_label(label)
    );
}

#[test]
fn test_label_matches() {
    let test_text = r#"A link that [Matches] and a [missing][missing link].
        
Here a [working    link][].
    
[matches]: https://match.es

[working link]: https://worki.ng"#;

    assert!(label_matches("Matches", test_text));
    assert!(!label_matches("Matches1", test_text));
    assert!(!label_matches("Matches 1", test_text));
    assert!(!label_matches("missing link", test_text));
    assert!(label_matches("WORKING LINK", test_text));
    assert!(label_matches("working   link", test_text));
}
