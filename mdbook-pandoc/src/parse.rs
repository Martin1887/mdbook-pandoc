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
    book::{parse_summary, Summary, SummaryItem},
    renderer::RenderContext,
};
use std::{
    fs,
    fs::File,
    io::{Read, Write},
    path::Path,
    path::PathBuf,
};

use crate::config::TitleLabels;

/// Label appended to headers to add the classes `.unnumbered` and `.unlisted`
const UNNUMBERED_UNLISTED: &'static str = " {.unnumbered .unlisted}";

/// Type of MD header: Atx (#), Setext1 (====) or Setext2(----).
enum HeaderType {
    Atx,
    Setext1,
    Setext2,
}

/// Return the header type of the line or `None` if it is not a header.
fn header_type(line: &str, next_line: &str) -> Option<HeaderType> {
    if line.starts_with("#") {
        Some(HeaderType::Atx)
    } else {
        match next_line.chars().next() {
            Some('=') => Some(HeaderType::Setext1),
            Some('-') => Some(HeaderType::Setext2),
            _ => None,
        }
    }
}

/// Return the new level for the header adding the current level
/// and the hierarchy level.
fn new_header_level(line: &str, hierarchy_level: usize, header_type: HeaderType) -> usize {
    let current_level: usize = match header_type {
        // Atx header has priority over Setext headers, ignoring Setext headers
        // as a normal line if Atx
        HeaderType::Atx => {
            let mut level = 0;
            for current in line.chars() {
                if current == '#' {
                    level += 1;
                } else {
                    break;
                }
            }
            level
        }
        HeaderType::Setext1 => 1,
        HeaderType::Setext2 => 2,
    };

    current_level + hierarchy_level
}

/// Return the transformed `line` and `next_line` analyzing header patterns.
/// The `new_line` is the same than original unless it defines a setext header
/// and the `line` header is returned changing its level and appending
/// '.unnumbered' and '.unlisted' classes to remove numbers and entries from TOC.
///
/// # Parameters
/// - `line`: The line of the header.
/// - `next_line`: The next line to check Setext headers.
/// - `hierarchy_level`: The hierarchy level to be added to the current level header.
/// - `first_transform`: `bool` to not remove from the table of contents the first header.
fn transform_header<'a>(
    line: &str,
    next_line: &'a str,
    hierarchy_level: usize,
    first_transform: bool,
) -> (String, &'a str, bool) {
    match header_type(&line, &next_line) {
        None => {
            // Unmodified lines
            (String::from(line), next_line, false)
        }
        Some(header_type) => {
            let transformed_next_line = match header_type {
                HeaderType::Atx => next_line,
                _ => "",
            };
            let new_header_level = new_header_level(&line, hierarchy_level, header_type);
            let clean_line = String::from(line.replace("#", "").trim());
            let transformed_line = if new_header_level > 6 {
                // Markdown only supports 6 levels, so following levels are coded as
                // simply bold text
                format!("**{}**\n", clean_line.trim())
            } else {
                // The first transformation does not remove the numeration because
                // it is the section title
                format!(
                    "{} {}{}",
                    "#".repeat(new_header_level),
                    clean_line.trim(),
                    if first_transform {
                        ""
                    } else {
                        UNNUMBERED_UNLISTED
                    }
                )
            };

            (transformed_line, transformed_next_line, true)
        }
    }
}

/// Transform the full Markdown file iterating over pairs of lines (to handle
/// setext headers) and transforming them setting classes and level.
fn transform_md(md: &str, level: usize) -> String {
    let mut formatted = String::from("\n");
    // Iterate over lines saving the previous line to find headers
    // (Setext headers are defined using '=' or '-' in the next line)
    // and writing the transformed `prev_line` in `formatted`.
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

/// Return the `Summary` object searching the file from the root path and
/// parsing it.
fn get_summary(root_path: &Path) -> Summary {
    let mut summary_path = root_path.to_owned().clone();
    summary_path.push("src");
    summary_path.push("SUMMARY.md");
    let mut summary_text = String::new();
    File::open(&summary_path)
        .expect("Failed to open SUMMARY.md file")
        .read_to_string(&mut summary_text)
        .expect("Failed to read SUMMARY.md file");
    parse_summary(&summary_text).expect("Failed to parse SUMMARY.md file")
}

/// Return `true` if the chapters contains at least one part and `false` otherwise.
fn chapters_have_parts(summary: &Summary) -> bool {
    summary
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
        > 0
}

/// The initial hierarchy level is 1 if there are prefixes, suffixes or the
/// chapters have parts and 0 otherwise.
fn initial_hierarchy_level(summary: &Summary, chapters_have_parts: bool) -> usize {
    if !summary.prefix_chapters.is_empty()
        || !summary.suffix_chapters.is_empty()
        || chapters_have_parts
    {
        1
    } else {
        0
    }
}

/// Parse the book contents and return the result of parse all chapters.
fn parse_book_contents(
    root_path: &Path,
    summary: &Summary,
    chapters_have_parts: bool,
    initial_level: usize,
    title_labels: &TitleLabels,
) -> String {
    let mut src_path = root_path.to_owned().clone();
    src_path.push("src");

    let mut parsed_content = String::new();
    if !summary.prefix_chapters.is_empty() {
        parsed_content.push_str(&format!("\n# {}", title_labels.preamble));
        parsed_content.push_str(&recursively_concatenate_book(
            &summary.prefix_chapters,
            &src_path,
            initial_level,
        ));
    }
    if !chapters_have_parts {
        parsed_content.push_str(&format!("\n# {}", title_labels.chapters));
    }
    parsed_content.push_str(&recursively_concatenate_book(
        &summary.numbered_chapters,
        &src_path,
        initial_level,
    ));
    if !summary.suffix_chapters.is_empty() {
        parsed_content.push_str(&format!("\n# {}", title_labels.annexes));
        parsed_content.push_str(&recursively_concatenate_book(
            &summary.suffix_chapters,
            &src_path,
            initial_level,
        ));
    }

    parsed_content
}

/// Write the parsed contents into the Pandoc MD file and return that path
/// (`./book/pandoc/md/book.md file`)
fn write_pandoc_md_file(root_path: &Path, parsed_content: &str) -> PathBuf {
    let mut pandoc_path = root_path.to_owned().clone();
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

/// Parse a full mdBook getting (and adding) parts and concatenating prefixes,
/// numbered chapters and suffixes, writing the contents in the file
/// `book/pandoc/md/book.md` and returning that path.
pub(crate) fn parse_book(ctx: &RenderContext, title_labels: &TitleLabels) -> PathBuf {
    let summary = get_summary(&ctx.root);
    let chapters_have_parts = chapters_have_parts(&summary);
    let initial_level = initial_hierarchy_level(&summary, chapters_have_parts);

    let parsed_contents = parse_book_contents(
        &ctx.root,
        &summary,
        chapters_have_parts,
        initial_level,
        title_labels,
    );

    write_pandoc_md_file(&ctx.root, &parsed_contents)
}
