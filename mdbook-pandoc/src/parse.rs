//! Parse the source Markdown files concatenating them and modifying the levels
//! of headers:
//! - If the book have prefixes or suffixes a part is created for each of them.
//! - If parts are created for prefixes and/or suffixes and the numbered
//! chapters don't have parts then a part is created for the chapters, the
//! original parts are used for the numbered chapters otherwise.
//! - The headers of the files are downgraded adding the level of the file
//! (e.g. a chapter inside a part has level 1 so its 1-level header becomes 2-level).
//! - If a header's level becomes bigger than 6 then the text is simply bold.
//! - Setext headers (underlined ones) are changed to atx (hashes prefix).
//! - Headers that are not the main one are labeled with `{.unnumbered .unlisted}`
//! to remove numbers and avoid that they appear in the table of contents.
//! - Each Markdown file must have a main 1st level header as the first header
//! of the file and that header is used as its header, the name of the file is
//! ignored.

use mdbook::{
    book::{parse_summary, SummaryItem},
    renderer::RenderContext,
};
use std::{
    fs,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

/// Label appended to headers to add the classes `.unnumbered` and `.unlisted`
const UNNUMBERED_UNLISTED: &'static str = " {.unnumbered .unlisted}";

/// Return the transformed `line` and `next_line` analyzing header patterns.
/// The `new_line` is the same than original unless it defines a setext header
/// and the `line` header is returned changing its level and appending
/// '.unnumbered' and '.unlisted' classes to remove numbers and from TOC.
fn transform_header<'a>(
    line: &str,
    next_line: &'a str,
    hierarchy_level: usize,
    first_transform: bool,
) -> (String, &'a str, bool) {
    let atx_header = line.starts_with("#");
    let setext1_header = next_line.starts_with("=");
    let setext2_header = next_line.starts_with("-");
    // atx header has priority over setext headers, ignoring setext headers as
    // a normal line if atx
    if atx_header || setext1_header || setext2_header {
        let mut transformed_next_line = next_line;
        let current_level = if atx_header {
            let mut level = 0;
            for current in line.chars() {
                if current == '#' {
                    level += 1;
                } else {
                    break;
                }
            }
            level
        } else {
            transformed_next_line = "";
            if setext1_header {
                1
            } else {
                2
            }
        };
        let new_level = current_level + hierarchy_level;
        let clean_line = String::from(line.replace("#", "").trim());
        let transformed_line = if new_level > 6 {
            // Markdown only supports 6 levels, so following levels are coded as
            // simply bold text
            format!("**{}**\n", clean_line.trim())
        } else {
            // The first transformation does not remove the numeration because
            // it is the section title
            format!(
                "{} {} {}",
                "#".repeat(new_level),
                clean_line.trim(),
                if first_transform {
                    ""
                } else {
                    UNNUMBERED_UNLISTED
                }
            )
        };

        (transformed_line, transformed_next_line, true)
    } else {
        (String::from(line), next_line, false)
    }
}

/// Transform the full Markdown file iterating over pairs of lines (to handle
/// setext headers) and transforming them setting classes and level.
fn transform_md(md: &str, level: usize) -> String {
    let mut formatted = String::from("\n");
    // iterate over lines saving the previous line to find headers
    // (Setext headers are defined using '=' or '-' in the next line)
    // and writing the transformed `prev_line` in `formatted`.
    // A new empty line is added at the end to cover all lines in the loop
    let mut lines = md.lines().chain("\n".lines());
    let mut prev_line: &str = lines.next().unwrap_or("");
    let mut first_transform = true;
    for line in lines {
        let (transformed_line, new_line, is_transformed) =
            transform_header(&prev_line, &line, level, first_transform);
        formatted.push_str(&format!("{}\n", transformed_line));
        prev_line = new_line;
        if is_transformed {
            first_transform = false;
        }
    }

    formatted
}

/// Read the Markdown file in memory and transform it.
fn read_and_transform_md(src_path: &PathBuf, file_path: &PathBuf, level: usize) -> String {
    let mut orig_contents = String::new();
    let mut path = src_path.clone();
    path.push(file_path);
    File::open(path.clone())
        .expect(&format!("Failed to open file {:?}", path))
        .read_to_string(&mut orig_contents)
        .expect(&format!("Error reading file {:?}", path));

    transform_md(&orig_contents, level)
}

/// Concatenate recursively a list of `SummaryItem` (with their children).
fn recursively_concatenate_book(items: &[SummaryItem], src_path: &PathBuf, level: usize) -> String {
    let mut parsed_contents = String::new();
    for item in items {
        parsed_contents.push_str("\n");
        match item {
            SummaryItem::Link(ref link) => {
                parsed_contents.push_str(&read_and_transform_md(
                    src_path,
                    &(link.location.clone().unwrap()),
                    level,
                ));
                parsed_contents.push_str(&recursively_concatenate_book(
                    &link.nested_items,
                    src_path,
                    level + 1,
                ));
            }
            // Separators are ignored, they make no sense in documents
            SummaryItem::Separator => (),
            SummaryItem::PartTitle(title) => {
                parsed_contents.push_str(&format!("\n# {}", title));
            }
        }
    }

    parsed_contents
}

/// Parse a full mdBook getting (and adding) parts and concatenating prefixes,
/// numbered chapters and suffixes, writing the contents in the file
/// `book/pandoc/md/book.md` and returning that path.
pub(crate) fn parse_book(
    ctx: &RenderContext,
    preamble_text: &str,
    chapters_text: &str,
    annexes_text: &str,
) -> PathBuf {
    let mut summary_path = ctx.root.clone();
    summary_path.push("src");
    summary_path.push("SUMMARY.md");
    let mut summary_text = String::new();
    File::open(&summary_path)
        .expect("Failed to open SUMMARY.md file")
        .read_to_string(&mut summary_text)
        .expect("Failed to read SUMMARY.md file");
    let summary = parse_summary(&summary_text).expect("Failed to parse SUMMARY.md file");

    let chapters_have_parts = summary
        .numbered_chapters
        .iter()
        .filter(|ch| -> bool {
            match ch {
                SummaryItem::Link(_) => false,
                SummaryItem::Separator => false,
                SummaryItem::PartTitle(_) => true,
            }
        })
        .count()
        > 0;
    let level: usize = if !summary.prefix_chapters.is_empty()
        || !summary.suffix_chapters.is_empty()
        || chapters_have_parts
    {
        1
    } else {
        0
    };

    let mut src_path = ctx.root.clone();
    src_path.push("src");

    let mut parsed_content = String::new();
    if !summary.prefix_chapters.is_empty() {
        parsed_content.push_str(&format!("\n# {}", preamble_text));
        parsed_content.push_str(&recursively_concatenate_book(
            &summary.prefix_chapters,
            &src_path,
            level,
        ));
    }
    if !chapters_have_parts {
        parsed_content.push_str(&format!("\n# {}", chapters_text));
    }
    parsed_content.push_str(&recursively_concatenate_book(
        &summary.numbered_chapters,
        &src_path,
        level,
    ));
    if !summary.suffix_chapters.is_empty() {
        parsed_content.push_str(&format!("\n# {}", annexes_text));
        parsed_content.push_str(&recursively_concatenate_book(
            &summary.suffix_chapters,
            &src_path,
            level,
        ));
    }

    // Write the contents into book/pandoc/md/book.md file
    let mut pandoc_path = ctx.root.clone();
    pandoc_path.push("book");
    pandoc_path.push("pandoc");
    let mut md_path = pandoc_path.clone();
    md_path.push("md");
    // Create the output directory
    fs::create_dir_all(&md_path).expect("Error creating the output directory");

    // Write the file
    md_path.push("book.md");
    let mut md_out = File::create(&md_path).expect("Error writing the parsed MD file");
    md_out
        .write_all(parsed_content.as_bytes())
        .expect("Error writing the parsed MD File");

    pandoc_path
}
