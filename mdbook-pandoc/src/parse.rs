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

/// Preprocessing functions for headers, delimiters and paths.
mod preprocessor;
#[cfg(test)]
mod tests;

use mdbook::{
    book::{parse_summary, Summary, SummaryItem},
    renderer::RenderContext,
    BookItem,
};
use std::{
    collections::HashSet,
    env::set_current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::config::TitleLabels;
use preprocessor::*;

/// Transform the full Markdown file following these steps:
/// 1. Replace all the SETEXT headings by the equivalent ATX headers via regex.
/// 2. Write the mark indicating the beginning of the MD file.
/// 3. Iterate over lines to transform the header into the correct level and
/// adding identifiers and classes.
/// 4. Write the mark indicating the end of the MD file.
/// 5. Replace the mdBook math delimiters by the Pandoc ones.
/// 6. Replace the mdBook custom code block annotations by nothing
/// (not supported by Pandoc) .
/// 7. Translate relative paths to start in the `src` folder.
/// 8. Fix the internal links using the unique identifiers and removing the
/// wrong ones with a warning.
fn transform_md(
    md: &str,
    level: usize,
    source_path: &Option<PathBuf>,
    book_paths: &Box<HashSet<PathBuf>>,
    mut section_number: &mut Vec<u32>,
) -> String {
    let source_path_str = source_path.clone().unwrap().to_str().unwrap().to_string();
    let mut formatted = format!("\n<!-- {} begins -->\n\n", source_path_str);
    let md_atx_only = setext2atx(md);
    let lines = md_atx_only.lines().chain("\n".lines());
    let mut first_transform = true;
    for line in lines {
        let (transformed_line, is_transformed) =
            transform_header(&line, level, first_transform, &mut section_number);
        formatted.push_str(&format!("{}\n", transformed_line));

        if is_transformed {
            first_transform = false;
        }
    }

    formatted.push_str(&format!("\n<!-- {} ends -->\n", source_path_str));

    formatted = replace_math_delimiters(&formatted);
    formatted = replace_custom_mdbook_code_block_annotations(&formatted);
    formatted = translate_relative_paths(&formatted, source_path, book_paths);
    formatted = fix_internal_links(&formatted);

    formatted
}

/// Concatenate recursively a list of `BookItem` (with their children) and
/// parse headers, delimiters and paths.
fn recursively_concatenate_book(
    items: &[BookItem],
    level: usize,
    book_paths: &Box<HashSet<PathBuf>>,
    section_number: &mut Vec<u32>,
) -> String {
    let mut parsed_contents = String::new();
    for item in items {
        parsed_contents.push_str("\n");
        match item {
            BookItem::Chapter(ref c) => {
                parsed_contents.push_str(&transform_md(
                    &c.content,
                    level,
                    &c.source_path,
                    book_paths,
                    section_number,
                ));
                parsed_contents.push_str(&recursively_concatenate_book(
                    &c.sub_items,
                    level + 1,
                    book_paths,
                    section_number,
                ));
            }
            // Separators are ignored, they make no sense in documents
            BookItem::Separator => (),
            BookItem::PartTitle(title) => {
                parsed_contents.push_str(&format_part_header(title, section_number));
            }
        }
    }

    parsed_contents
}

/// Return the `Summary` object searching the file from the root src and
/// parsing it.
fn get_summary(src_path: &Path) -> Summary {
    let summary_path = src_path.join("SUMMARY.md");
    let mut summary_text = String::new();
    File::open(&summary_path)
        .expect("Failed to open SUMMARY.md file")
        .read_to_string(&mut summary_text)
        .expect("Failed to read SUMMARY.md file");
    parse_summary(&summary_text).expect("Failed to parse SUMMARY.md file")
}

/// Return `true` if the chapters contains at least one part and `false` otherwise.
fn chapters_have_parts(book_items: &[BookItem]) -> bool {
    book_items
        .iter()
        .filter(|ch| -> bool {
            match ch {
                BookItem::Chapter(_) => false,
                BookItem::Separator => false,
                BookItem::PartTitle(_) => true,
            }
        })
        .count()
        > 0
}

/// Map a collection of chapters and parts into their names or nothing for separators
fn map_chapters_parts_names(chapters: &[SummaryItem]) -> Vec<String> {
    chapters
        .iter()
        .filter(|i| match i {
            SummaryItem::Separator => false,
            _ => true,
        })
        .map(|i| match i {
            SummaryItem::Link(l) => l.name.clone(),
            SummaryItem::PartTitle(t) => t.to_string(),
            SummaryItem::Separator => "".to_string(),
        })
        .collect()
}

/// Map a book item into its name or nothing if it is a separator
fn book_item_name(item: &BookItem) -> String {
    match item {
        BookItem::Chapter(c) => c.name.clone(),
        BookItem::Separator => String::from(""),
        BookItem::PartTitle(t) => t.to_owned(),
    }
}

/// Get the chapters corresponding to prefix, numbered or suffix using the
/// summary information
fn filter_per_chapter_type(
    book_items: &[BookItem],
    summary_type_chapters: &[SummaryItem],
) -> Vec<BookItem> {
    book_items
        .iter()
        .filter(|item| {
            map_chapters_parts_names(summary_type_chapters).contains(&book_item_name(item))
        })
        .map(|i| i.to_owned())
        .collect()
}

/// Analyze the summary returning the first hierarchy level, the prefix chapters,
/// the content chapters and the suffix chapters.
/// The initial hierarchy level is 1 if there are prefixes, suffixes or the
/// chapters have parts and 0 otherwise.
fn analyze_summary(
    summary: &Summary,
    book_items: &[BookItem],
    chapters_have_parts: bool,
) -> (
    usize,
    Vec<BookItem>,
    Vec<BookItem>,
    Vec<BookItem>,
    HashSet<PathBuf>,
) {
    let initial_hierarchy = if !summary.prefix_chapters.is_empty()
        || !summary.suffix_chapters.is_empty()
        || chapters_have_parts
    {
        1
    } else {
        0
    };

    let prefix_chapters = filter_per_chapter_type(book_items, &summary.prefix_chapters);
    let content_chapters = filter_per_chapter_type(book_items, &summary.numbered_chapters);
    let suffix_chapters = filter_per_chapter_type(book_items, &summary.suffix_chapters);

    let book_paths = book_items
        .iter()
        .filter(|c| match c {
            BookItem::Chapter(_) => true,
            _ => false,
        })
        .map(|c| match c {
            BookItem::Chapter(ref chapter) => {
                if let Some(path) = &chapter.source_path {
                    path.to_path_buf()
                        .canonicalize()
                        .expect("Error canonicalizing a book path")
                } else {
                    PathBuf::new()
                }
            }
            _ => PathBuf::new(),
        })
        .collect();

    (
        initial_hierarchy,
        prefix_chapters,
        content_chapters,
        suffix_chapters,
        book_paths,
    )
}

/// Format a part header.
fn format_part_header(text: &str, section_number: &mut Vec<u32>) -> String {
    update_section_number(section_number, 1);
    format!(
        "\n# {} {{#{}}}",
        text,
        header_identifier(text, section_number)
    )
}

fn write_chapters_header(
    parsed_content: &mut String,
    mut section_number: &mut Vec<u32>,
    chapters_label: &str,
    chapters_have_parts: bool,
    empty_prefixes: bool,
    empty_suffixes: bool,
) {
    if (!empty_prefixes || !empty_suffixes) && !chapters_have_parts {
        parsed_content.push_str(&format_part_header(&chapters_label, &mut section_number));
    }
}

/// Parse the book contents and return the result of parse all chapters.
fn parse_book_contents(
    prefix_chapters: &[BookItem],
    content_chapters: &[BookItem],
    suffix_chapters: &[BookItem],
    book_paths: &Box<HashSet<PathBuf>>,
    chapters_have_parts: bool,
    initial_level: usize,
    title_labels: &TitleLabels,
) -> String {
    // The whole document section number is initialized to 0 in order the first
    // header is 1
    let mut section_number = vec![0];
    let mut parsed_content = String::new();
    if !prefix_chapters.is_empty() {
        parsed_content.push_str(&format_part_header(
            &title_labels.preamble,
            &mut section_number,
        ));
        parsed_content.push_str(&recursively_concatenate_book(
            prefix_chapters,
            initial_level,
            book_paths,
            &mut section_number,
        ));
    }
    write_chapters_header(
        &mut parsed_content,
        &mut section_number,
        &title_labels.chapters,
        chapters_have_parts,
        prefix_chapters.is_empty(),
        suffix_chapters.is_empty(),
    );
    parsed_content.push_str(&recursively_concatenate_book(
        content_chapters,
        initial_level,
        &book_paths,
        &mut section_number,
    ));
    if !suffix_chapters.is_empty() {
        parsed_content.push_str(&format_part_header(
            &title_labels.annexes,
            &mut section_number,
        ));
        parsed_content.push_str(&recursively_concatenate_book(
            suffix_chapters,
            initial_level,
            &book_paths,
            &mut section_number,
        ));
    }

    parsed_content = fix_external_links(&parsed_content);

    parsed_content
}

/// Parse a full mdBook getting (and adding) parts and concatenating prefixes,
/// numbered chapters and suffixes, returning the result as String. writing the contents in the file
/// `book/pandoc/md/book.md` and returning that path.
pub(crate) fn parse_book(ctx: &RenderContext, title_labels: &TitleLabels) -> String {
    // Set the current working directory to the `src` path (everything is relative to the summary)
    let src_path = ctx.root.join("src");
    set_current_dir(&src_path).expect("Error setting the current dir to root path");

    // `BookItems` inner reference is problematic and `.sections` return all sections
    // instead of only the first-depth ones, so this is convenient
    let book_items: Vec<BookItem> = ctx.book.iter().map(|i| i.clone()).collect();
    let chapters_have_parts = chapters_have_parts(&book_items);
    let summary = get_summary(&src_path);
    let (initial_level, prefix_chapters, content_chapters, suffix_chapters, book_paths) =
        analyze_summary(&summary, &book_items, chapters_have_parts);

    parse_book_contents(
        &prefix_chapters,
        &content_chapters,
        &suffix_chapters,
        &Box::new(book_paths),
        chapters_have_parts,
        initial_level,
        title_labels,
    )
}
