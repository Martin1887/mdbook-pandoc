use std::collections::HashMap;

use super::*;
use mdbook::book::{Link, Summary};

fn get_test_summaries() -> HashMap<&'static str, Summary> {
    let mut map = HashMap::new();

    map.insert(
        "without_parts",
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
    );

    map.insert(
        "with_parts",
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
    );

    map.insert(
        "with_prefix",
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
    );

    map.insert(
        "with_suffix",
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
    );

    map.insert(
        "with_parts_and_suffix",
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
    );

    map.insert(
        "with_prefix_and_suffix",
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
    let summaries = get_test_summaries();
    assert_eq!(chapters_have_parts(&summaries["without_parts"]), false);
    assert_eq!(chapters_have_parts(&summaries["with_parts"]), true);
    assert_eq!(chapters_have_parts(&summaries["with_prefix"]), false);
    assert_eq!(chapters_have_parts(&summaries["with_suffix"]), false);
    assert_eq!(
        chapters_have_parts(&summaries["with_parts_and_suffix"]),
        true
    );
    assert_eq!(
        chapters_have_parts(&summaries["with_prefix_and_suffix"]),
        false
    );
}

#[test]
fn test_initial_hierarchy_level() {
    let summaries = get_test_summaries();
    assert_eq!(
        initial_hierarchy_level(&summaries["without_parts"], false),
        0
    );
    assert_eq!(initial_hierarchy_level(&summaries["with_parts"], true), 1);
    assert_eq!(initial_hierarchy_level(&summaries["with_prefix"], false), 1);
    assert_eq!(initial_hierarchy_level(&summaries["with_suffix"], false), 1);
    assert_eq!(
        initial_hierarchy_level(&summaries["with_parts_and_suffix"], true),
        1
    );
    assert_eq!(
        initial_hierarchy_level(&summaries["with_prefix_and_suffix"], true),
        1
    );
}
