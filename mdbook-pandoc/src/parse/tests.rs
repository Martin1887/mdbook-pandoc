use std::collections::HashMap;

use super::*;
use mdbook::book::{Chapter, Link, Summary};

/// Return a HashMap of test vec of `BookItem` representing the first level of
/// the book and their summary.
fn get_test_data() -> HashMap<&'static str, (Vec<BookItem>, Summary)> {
    let mut map = HashMap::new();
    let dummy_chapter = BookItem::Chapter(Chapter::new(
        "Link",
        String::from("Link"),
        "None",
        Vec::new(),
    ));
    let dummy_prefix = BookItem::Chapter(Chapter::new(
        "Prefix Link",
        String::from("Prefix Link"),
        "None",
        Vec::new(),
    ));
    let dummy_suffix = BookItem::Chapter(Chapter::new(
        "Suffix Link",
        String::from("Suffix Link"),
        "None",
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
fn test_header_type() {
    assert!(
        matches!(header_type("# Title", ""), Some(HeaderType::Atx)),
        "Fail detecting Atx header"
    );
    assert!(
        matches!(
            header_type("## Subtitle ##", "Other things"),
            Some(HeaderType::Atx)
        ),
        "Fail detecting Atx header"
    );
    assert!(
        matches!(header_type("Title", "===="), Some(HeaderType::Setext1)),
        "Fail detecting Setext header of level 1"
    );
    assert!(
        matches!(
            header_type("Subtitle", "-------------"),
            Some(HeaderType::Setext2)
        ),
        "Fail detecting Setext header of level 2"
    );
    assert!(
        matches!(header_type("Thing", "Other things"), None),
        "Fail detecting the line is not header"
    );
}
#[test]
fn test_new_header_level() {
    assert_eq!(new_header_level("# Title", 0, HeaderType::Atx), 1);
    assert_eq!(new_header_level("# Title", 2, HeaderType::Atx), 3);
    assert_eq!(new_header_level("## Title", 0, HeaderType::Atx), 2);
    assert_eq!(new_header_level("## Title", 1, HeaderType::Atx), 3);
    assert_eq!(new_header_level("Title", 0, HeaderType::Setext1), 1);
    assert_eq!(new_header_level("Title", 1, HeaderType::Setext1), 2);
    assert_eq!(new_header_level("Title", 0, HeaderType::Setext2), 2);
    assert_eq!(new_header_level("Title", 4, HeaderType::Setext2), 6);
    assert_eq!(new_header_level("##### Title", 2, HeaderType::Atx), 7);
}

#[test]
fn test_transform_header() {
    assert_eq!(
        transform_header("# Title", "Other things", 1, false),
        (format!("## Title{}", UNNUMBERED_UNLISTED), false, true)
    );
    assert_eq!(
        transform_header("# Title", "Other things", 1, true),
        (String::from("## Title"), false, true)
    );
    assert_eq!(
        transform_header("Title", "======", 1, true),
        (String::from("## Title"), true, true)
    );
    assert_eq!(
        transform_header("Things", "Other things", 1, true),
        (String::from("Things"), false, false)
    );
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
