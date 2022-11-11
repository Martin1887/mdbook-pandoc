use std::{collections::HashMap, sync::Once};

use super::*;
use mdbook::book::{Chapter, Link, Summary};

/// Unique `Once` to set the current test dir by relative path only once in all
/// tests.
pub(crate) static INIT: Once = Once::new();

/// Return a HashMap of test vec of `BookItem` representing the first level of
/// the book and their summary.
fn get_test_data() -> HashMap<&'static str, (Vec<BookItem>, Summary)> {
    INIT.call_once(|| {
        set_current_dir("assets/tests/test_book/src").expect("Error setting the current directory");
    });

    let mut map = HashMap::new();
    let dummy_chapter = BookItem::Chapter(Chapter::new(
        "Link",
        String::from("Link"),
        "Introduction/README.md",
        Vec::new(),
    ));
    let dummy_prefix = BookItem::Chapter(Chapter::new(
        "Prefix Link",
        String::from("Prefix Link"),
        "Introduction/README.md",
        Vec::new(),
    ));
    let dummy_suffix = BookItem::Chapter(Chapter::new(
        "Suffix Link",
        String::from("Suffix Link"),
        "Introduction/README.md",
        Vec::new(),
    ));
    let dummy_part = BookItem::PartTitle(String::from("Part 1"));

    map.insert(
        "without_parts",
        (
            vec![dummy_chapter.clone()],
            Summary {
                title: Some(String::from("Title")),
                prefix_chapters: Vec::new(),
                numbered_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
                suffix_chapters: Vec::new(),
            },
        ),
    );

    map.insert(
        "with_parts",
        (
            vec![dummy_part.clone(), dummy_chapter.clone()],
            Summary {
                title: Some(String::from("Title")),
                prefix_chapters: Vec::new(),
                numbered_chapters: vec![
                    SummaryItem::Link(Link {
                        name: String::from("Link"),
                        location: None,
                        number: None,
                        nested_items: Vec::new(),
                    }),
                    SummaryItem::PartTitle(String::from("Part 1")),
                ],
                suffix_chapters: Vec::new(),
            },
        ),
    );

    map.insert(
        "with_prefix",
        (
            vec![dummy_prefix.clone(), dummy_chapter.clone()],
            Summary {
                title: Some(String::from("Title")),
                prefix_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Prefix Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
                numbered_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
                suffix_chapters: Vec::new(),
            },
        ),
    );

    map.insert(
        "with_suffix",
        (
            vec![dummy_chapter.clone(), dummy_suffix.clone()],
            Summary {
                title: Some(String::from("Title")),
                prefix_chapters: Vec::new(),
                numbered_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
                suffix_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Suffix Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
            },
        ),
    );

    map.insert(
        "with_parts_and_suffix",
        (
            vec![
                dummy_part.clone(),
                dummy_chapter.clone(),
                dummy_suffix.clone(),
            ],
            Summary {
                title: Some(String::from("Title")),
                prefix_chapters: Vec::new(),
                numbered_chapters: vec![
                    SummaryItem::Link(Link {
                        name: String::from("Link"),
                        location: None,
                        number: None,
                        nested_items: Vec::new(),
                    }),
                    SummaryItem::PartTitle(String::from("Part 1")),
                ],
                suffix_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Suffix Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
            },
        ),
    );

    map.insert(
        "with_prefix_and_suffix",
        (
            vec![
                dummy_prefix.clone(),
                dummy_chapter.clone(),
                dummy_suffix.clone(),
            ],
            Summary {
                title: Some(String::from("Title")),
                prefix_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Prefix Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
                numbered_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
                suffix_chapters: vec![SummaryItem::Link(Link {
                    name: String::from("Suffix Link"),
                    location: None,
                    number: None,
                    nested_items: Vec::new(),
                })],
            },
        ),
    );

    map
}

#[test]
fn test_chapters_have_parts() {
    let data = get_test_data();
    assert_eq!(chapters_have_parts(&data["without_parts"].0), false);
    assert_eq!(chapters_have_parts(&data["with_parts"].0), true);
    assert_eq!(chapters_have_parts(&data["with_prefix"].0), false);
    assert_eq!(chapters_have_parts(&data["with_suffix"].0), false);
    assert_eq!(chapters_have_parts(&data["with_parts_and_suffix"].0), true);
    assert_eq!(
        chapters_have_parts(&data["with_prefix_and_suffix"].0),
        false
    );
}

#[test]
fn test_initial_hierarchy_level() {
    let data = get_test_data();
    assert_eq!(
        analyze_summary(&data["without_parts"].1, &data["without_parts"].0, false).0,
        0
    );
    assert_eq!(
        analyze_summary(&data["with_parts"].1, &data["with_parts"].0, true).0,
        1
    );
    assert_eq!(
        analyze_summary(&data["with_prefix"].1, &data["with_prefix"].0, false).0,
        1
    );
    assert_eq!(
        analyze_summary(&data["with_suffix"].1, &data["with_suffix"].0, false).0,
        1
    );
    assert_eq!(
        analyze_summary(
            &data["with_parts_and_suffix"].1,
            &data["with_parts_and_suffix"].0,
            true
        )
        .0,
        1
    );
    assert_eq!(
        analyze_summary(
            &data["with_prefix_and_suffix"].1,
            &data["with_prefix_and_suffix"].0,
            true
        )
        .0,
        1
    );
}

#[test]
fn test_write_chapters_header() {
    let mut parsed_content = String::new();
    let mut section_number = vec![0];
    let chapters_label = "Chapters";

    write_chapters_header(
        &mut parsed_content,
        &mut section_number,
        &chapters_label,
        true,
        true,
        true,
    );
    assert_eq!(parsed_content, String::new());

    parsed_content = String::new();
    write_chapters_header(
        &mut parsed_content,
        &mut section_number,
        &chapters_label,
        false,
        true,
        true,
    );
    assert_eq!(parsed_content, String::new());

    parsed_content = String::new();
    write_chapters_header(
        &mut parsed_content,
        &mut section_number,
        &chapters_label,
        false,
        false,
        true,
    );
    assert!(parsed_content.contains("Chapters"));

    parsed_content = String::new();
    write_chapters_header(
        &mut parsed_content,
        &mut section_number,
        &chapters_label,
        true,
        true,
        false,
    );
    assert_eq!(parsed_content, String::new());
}
