use super::super::*;
use super::*;
use mdbook::book::Chapter;
use pulldown_cmark::HeadingLevel::*;

#[test]
fn test_new_header_level() {
    assert_eq!(new_header_level(H1, 0), 1);
    assert_eq!(new_header_level(H1, 2), 3);
    assert_eq!(new_header_level(H2, 0), 2);
    assert_eq!(new_header_level(H2, 1), 3);
    assert_eq!(new_header_level(H2, 4), 6);
    assert_eq!(new_header_level(H5, 2), 7);
}

#[test]
fn test_transform_header() {
    let mut section_number = vec![1];
    let title_heading = HeadingAttrs {
        id: None,
        level: H1,
        text: "Title".to_string(),
        classes: Vec::new(),
    };
    assert_eq!(
        transform_header(&title_heading, 1, false, true, &mut section_number),
        format!(
            "## Title {{#{} {} {}}}\n",
            header_identifier("Title", &section_number),
            UNNUMBERED,
            UNLISTED
        )
    );
    assert_eq!(
        transform_header(&title_heading, 1, false, false, &mut section_number),
        format!(
            "## Title {{#{}}}\n",
            header_identifier("Title", &section_number),
        )
    );
    assert_eq!(
        transform_header(&title_heading, 1, true, true, &mut section_number),
        format!(
            "## Title {{#{}}}\n",
            header_identifier("Title", &section_number),
        )
    );

    assert_eq!(
        transform_header(
            &HeadingAttrs {
                id: None,
                level: H1,
                text: "Things".to_string(),
                classes: vec!["class1".to_string(), "class2".to_string()]
            },
            1,
            false,
            false,
            &mut section_number
        ),
        format!(
            "## Things {{#{} .class1 .class2}}\n",
            header_identifier("Things", &section_number)
        ),
        "Attributes not well captured"
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
