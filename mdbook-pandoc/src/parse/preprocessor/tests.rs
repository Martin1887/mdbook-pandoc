use super::super::*;
use super::*;
use mdbook::book::Chapter;

#[test]
fn test_atx_header() {
    assert!(
        matches!(is_atx_header("# Title {my_attr}"), true),
        "Fail detecting Atx header with attributes"
    );
    assert!(
        matches!(is_atx_header("## Subtitle ##"), true),
        "Fail detecting Atx header"
    );
    assert!(
        matches!(is_atx_header("#Title"), false),
        "Fail detecting no header starting with '#'"
    );
    assert!(
        matches!(is_atx_header(" ## Subtitle ##"), true),
        "Fail detecting header starting with space"
    );
    assert!(
        matches!(is_atx_header("   #### Section"), true),
        "Fail detecting header starting with three spaces"
    );
    assert!(
        matches!(is_atx_header("    # False title"), false),
        "Fail detecting no header starting with four spaces"
    );
    assert!(
        matches!(is_atx_header(" ####### Seventh level"), false),
        "Fail detecting no title with 7 '#"
    );
}

#[test]
fn test_setext_header() {
    assert_eq!(
        setext2atx("Title\n===="),
        String::from("# Title\n"),
        "Fail detecting Setext header of level 1"
    );
    assert_eq!(
        setext2atx("Paragraph\n\n Test\n  Subtitle with\nnewline\n-------------   "),
        String::from("Paragraph\n\n## Test   Subtitle with newline\n"),
        "Fail detecting Setext header of level 2 in several lines"
    );
    assert_eq!(
        setext2atx(">Quote\n Title with\nnewline\n   ===="),
        String::from(">Quote\n# Title with newline\n"),
        "Fail detecting Setext header of level 1 in several lines after quote"
    );
    let paragraph_and_list = String::from("Paragraph\n- List item");
    assert_eq!(
        setext2atx(&paragraph_and_list),
        paragraph_and_list,
        "List item detected as SETEXT header"
    );
    let things = String::from("Thing\nOther things");
    assert_eq!(
        setext2atx(&things),
        things,
        "Fail detecting the line is not header"
    );

    let yaml_block = String::from("---\nfield1 = true\nfield2 = things\n---");
    assert_eq!(
        setext2atx(&yaml_block),
        yaml_block,
        "YAML block detected as h2"
    );
    let yaml_block_with_final_empty_line = String::from("---\nfield1=true\n\n---");
    assert_eq!(
        setext2atx(&yaml_block_with_final_empty_line),
        yaml_block_with_final_empty_line,
        "YAML block detected as h2"
    );
    let paragraph_and_hr = String::from("Things\n\n---");
    assert_eq!(
        setext2atx(&paragraph_and_hr),
        paragraph_and_hr,
        "hr after blank lines detected as h2"
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
        transform_header("# Title", 1, false, true, &mut section_number),
        (
            format!(
                "## Title {{#{}{}}}",
                header_identifier("Title", &section_number),
                UNNUMBERED_UNLISTED
            ),
            true
        )
    );
    assert_eq!(
        transform_header("# Title", 1, false, false, &mut section_number),
        (
            format!(
                "## Title {{#{}}}",
                header_identifier("Title", &section_number),
            ),
            true
        )
    );
    assert_eq!(
        transform_header("# Title", 1, true, true, &mut section_number),
        (
            format!(
                "## Title {{#{}}}",
                header_identifier("Title", &section_number),
            ),
            true
        )
    );
    assert_eq!(
        transform_header("Things", 1, true, true, &mut section_number),
        (String::from("Things"), false)
    );

    assert_eq!(
        transform_header(
            "# Things # {attr1 attr2=val}",
            1,
            false,
            false,
            &mut section_number
        ),
        (
            format!(
                "## Things {{#{} attr1 attr2=val}}",
                header_identifier("Things", &section_number)
            ),
            true
        ),
        "Attributes not well captured"
    );

    assert_eq!(
        transform_header(
            "# Things  {attr1 attr2=val} #",
            1,
            false,
            false,
            &mut section_number
        ),
        (
            format!(
                "## Things  {{attr1 attr2=val}} {{#{}}}",
                header_identifier("Things  {{attr1 attr2=val}}", &section_number)
            ),
            true
        ),
        "Header text inside curly braces wrongly handled as attributes"
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
        &HashSet::new(),
        &mut section_number,
        true,
    );
    assert_eq!(section_number, vec![1, 1, 1]);
    recursively_concatenate_book(
        &[BookItem::Chapter(Chapter::new(
            "Subchapter",
            "# Subsection \n\nMore things.\n\n".to_string(),
            "path",
            vec![],
        ))],
        level + 1,
        &HashSet::new(),
        &mut section_number,
        false,
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
