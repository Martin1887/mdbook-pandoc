use super::super::*;
use super::*;
use mdbook::book::Chapter;

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
        matches!(header_type("#Title", ""), None),
        "Fail detecting no header starting with '#'"
    );
    assert!(
        matches!(header_type(" ## Subtitle", ""), Some(HeaderType::Atx)),
        "Fail detecting header starting with space"
    );
    assert!(
        matches!(header_type("   #### Section", ""), Some(HeaderType::Atx)),
        "Fail detecting header starting with three spaces"
    );
    assert!(
        matches!(header_type("    # False title", ""), None),
        "Fail detecting no header starting with four spaces"
    );
    assert!(
        matches!(header_type(" ####### Seventh level", ""), None),
        "Fail detecting no title with 7 '#"
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
        matches!(header_type("Paragraph", "- List item"), None),
        "List item detected as SETEXT header"
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
    let mut section_number = vec![1];
    assert_eq!(
        transform_header("# Title", "Other things", 1, false, &mut section_number),
        (
            format!(
                "## Title {{#{}{}}}",
                header_identifier("Title", &section_number),
                UNNUMBERED_UNLISTED
            ),
            false,
            true
        )
    );
    assert_eq!(
        transform_header("# Title", "Other things", 1, true, &mut section_number),
        (
            format!(
                "## Title {{#{}}}",
                header_identifier("Title", &section_number),
            ),
            false,
            true
        )
    );
    assert_eq!(
        transform_header("Title", "======", 1, true, &mut section_number),
        (
            format!(
                "## Title {{#{}}}",
                header_identifier("Title", &section_number),
            ),
            true,
            true
        )
    );
    assert_eq!(
        transform_header("Things", "Other things", 1, true, &mut section_number),
        (String::from("Things"), false, false)
    );
}

#[test]
fn test_chapter_change_section_number() {
    let level = 1;
    let mut section_number = vec![1];
    transform_md(
        "# Level 1 \n\nThings. \n\n## Level 2 \n\nThings\n\n",
        level,
        &Some(Path::new("src/1.md").to_path_buf()),
        &Box::new(HashSet::new()),
        &mut section_number,
    );
    assert_eq!(section_number, vec![1, 1, 1]);
    recursively_concatenate_book(
        &vec![BookItem::Chapter(Chapter::new(
            "Subchapter",
            "# Subsection \n\nMore things.\n\n".to_string(),
            "path",
            vec![],
        ))],
        level + 1,
        &Box::new(HashSet::new()),
        &mut section_number,
    );
    assert_eq!(section_number, vec![1, 1, 2])
}

#[test]
fn test_update_section_number() {
    let mut section_number = vec![0];

    update_section_number(&mut section_number, 1);
    assert_eq!(section_number, vec![1]);

    update_section_number(&mut section_number, 2);
    assert_eq!(section_number, vec![1, 1]);

    update_section_number(&mut section_number, 4);
    assert_eq!(section_number, vec![1, 1, 1, 1]);

    update_section_number(&mut section_number, 4);
    assert_eq!(section_number, vec![1, 1, 1, 2]);

    update_section_number(&mut section_number, 5);
    assert_eq!(section_number, vec![1, 1, 1, 2, 1]);

    update_section_number(&mut section_number, 9);
    assert_eq!(section_number, vec![1, 1, 1, 2, 1, 1, 1, 1, 1]);

    update_section_number(&mut section_number, 9);
    assert_eq!(section_number, vec![1, 1, 1, 2, 1, 1, 1, 1, 2]);

    update_section_number(&mut section_number, 9);
    assert_eq!(section_number, vec![1, 1, 1, 2, 1, 1, 1, 1, 3]);

    update_section_number(&mut section_number, 2);
    assert_eq!(section_number, vec![1, 2]);

    update_section_number(&mut section_number, 2);
    assert_eq!(section_number, vec![1, 3]);

    update_section_number(&mut section_number, 1);
    assert_eq!(section_number, vec![2]);

    update_section_number(&mut section_number, 2);
    assert_eq!(section_number, vec![2, 1]);

    update_section_number(&mut section_number, 2);
    assert_eq!(section_number, vec![2, 2]);

    update_section_number(&mut section_number, 3);
    assert_eq!(section_number, vec![2, 2, 1]);

    update_section_number(&mut section_number, 3);
    assert_eq!(section_number, vec![2, 2, 2]);
}

#[test]
fn test_header_identifier_sanitize() {
    assert_eq!(
        header_identifier_sanitize("Title with  spaces, 21^numbers, +things+ and_DASHES__yes"),
        "title-with-spaces-21-numbers-things-and-dashes-yes"
    );
}

#[test]
fn test_replace_math_delimiters() {
    assert_eq!("Without math", replace_math_delimiters("Without math"));
    // inline math
    assert_eq!(
        r"Inline math $x + 1$",
        replace_math_delimiters(r"Inline math \\( x + 1 \\)")
    );
    assert_eq!(
        r"Inline math $x + 1$",
        replace_math_delimiters(r"Inline math \\(   x + 1   \\)")
    );
    assert_eq!(
        r"Inline math $x + 1$",
        replace_math_delimiters(r"Inline math \\(x + 1\\)")
    );
    // display math
    assert_eq!(
        r"Inline math $$x + 1$$",
        replace_math_delimiters(r"Inline math \\[ x + 1 \\]")
    );
    assert_eq!(
        r"Inline math $$x + 1$$",
        replace_math_delimiters(r"Inline math \\[   x + 1   \\]")
    );
    assert_eq!(
        r"Inline math $$x + 1$$",
        replace_math_delimiters(r"Inline math \\[x + 1\\]")
    );
}
