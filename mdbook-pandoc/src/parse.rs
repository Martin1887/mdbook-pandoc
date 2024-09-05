//! Parse the Markdown source files concatenating them and modifying the levels
//! of headings as specified in the documentation.

/// Preprocessing functions for headings, delimiters and paths.
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
    ops::Range,
    path::{Path, PathBuf},
};

use crate::config::TitleLabels;
use preprocessor::*;

/// Transform the full Markdown file following these steps:
/// 1. Replace all the SETEXT headings by the equivalent ATX headings via regex.
/// 2. Write the mark indicating the beginning of the MD file.
/// 3. Iterate over lines to transform the heading into the correct level and
///    adding identifiers and classes.
/// 4. Write the mark indicating the end of the MD file.
/// 5. Replace the mdBook math delimiters by the Pandoc ones.
/// 6. Replace the mdBook custom code block annotations by nothing
///    (not supported by Pandoc) .
/// 7. Translate relative paths to start in the `src` folder.
/// 8. Fix the internal links using the unique identifiers and removing the
///    wrong ones with a warning.
fn transform_md(
    md: &str,
    hierarchy_level: usize,
    source_path: &Option<PathBuf>,
    book_paths: &HashSet<PathBuf>,
    section_number: &mut Vec<u32>,
    unlist_headings: bool,
    headings_auto_identifiers: bool,
) -> String {
    let source_path_str = source_path.clone().unwrap().to_str().unwrap().to_string();
    let begin_mark = format!("\n<!-- {} begins -->\n\n", source_path_str);
    let end_mark = &format!("\n<!-- {} ends -->\n", source_path_str);

    // Clone replacing line breaks by `\n`
    let mut headings_transformed: String = md
        .lines()
        .chain("\n".lines())
        .collect::<Vec<&str>>()
        .join("\n");

    // It is important to use the transformed String to get the correct ranges
    let headings: Vec<(Range<usize>, HeadingAttrs)> = get_headings(&headings_transformed);
    let mut first_transform = true;
    // ranges are changed after replacing, so after the first replace they must be updated
    let mut replace_range_offset = 0_i64;
    for (range, heading_attrs) in headings {
        let transformed_line = transform_heading(
            &heading_attrs,
            hierarchy_level,
            first_transform,
            unlist_headings,
            headings_auto_identifiers,
            section_number,
        );
        let fixed_range = Range {
            start: (range.start as i64 + replace_range_offset) as usize,
            end: (range.end as i64 + replace_range_offset) as usize,
        };
        headings_transformed.replace_range(fixed_range, &transformed_line);

        replace_range_offset += transformed_line.len() as i64;
        replace_range_offset -= range.len() as i64;

        first_transform = false;
    }

    let mut formatted = format!("{}{}{}", begin_mark, headings_transformed, end_mark);

    formatted = replace_math_delimiters(&formatted);
    formatted = replace_custom_mdbook_code_block_annotations(&formatted);
    formatted = translate_relative_paths(&formatted, source_path, book_paths);
    formatted = fix_internal_links(&formatted);
    check_ref_links(&formatted);

    formatted
}

/// Concatenate recursively a list of `BookItem` (with their children) and
/// parse headings, delimiters and paths.
fn recursively_concatenate_book(
    items: &[BookItem],
    level: usize,
    book_paths: &HashSet<PathBuf>,
    section_number: &mut Vec<u32>,
    unlist_headings: bool,
    headings_auto_identifiers: bool,
) -> (String, Vec<String>) {
    let mut source_paths = Vec::new();
    let mut parsed_contents = String::new();
    for item in items {
        parsed_contents.push('\n');
        match item {
            BookItem::Chapter(ref c) => {
                parsed_contents.push_str(&transform_md(
                    &c.content,
                    level,
                    &c.source_path,
                    book_paths,
                    section_number,
                    unlist_headings,
                    headings_auto_identifiers,
                ));
                let (sub_contents, sub_paths) = &recursively_concatenate_book(
                    &c.sub_items,
                    level + 1,
                    book_paths,
                    section_number,
                    unlist_headings,
                    headings_auto_identifiers,
                );
                parsed_contents.push_str(sub_contents);
                source_paths.push(c.source_path.clone().unwrap().to_str().unwrap().to_string());
                source_paths.extend_from_slice(sub_paths);
            }
            // Separators are ignored, they make no sense in documents
            BookItem::Separator => (),
            BookItem::PartTitle(title) => {
                parsed_contents.push_str(&format_part_heading(
                    title,
                    headings_auto_identifiers,
                    section_number,
                ));
            }
        }
    }

    (parsed_contents, source_paths)
}

/// Return the `Summary` object searching the file from the root src and
/// parsing it.
fn get_summary(src_path: &Path) -> Summary {
    let summary_path = src_path.join("SUMMARY.md");
    let mut summary_text = String::new();
    File::open(summary_path)
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
        .filter(|i| !matches!(i, SummaryItem::Separator))
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
        .filter(|c| matches!(c, BookItem::Chapter(_)))
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

/// Format a part heading.
fn format_part_heading(
    text: &str,
    headings_auto_identifiers: bool,
    section_number: &mut Vec<u32>,
) -> String {
    update_section_number(section_number, 1);
    let attributes = if headings_auto_identifiers {
        format!(" {{#{}}}", heading_identifier(text, section_number))
    } else {
        "".to_string()
    };
    format!("\n# {}{}", text, attributes)
}

fn write_chapters_heading(
    parsed_content: &mut String,
    headings_auto_identifiers: bool,
    section_number: &mut Vec<u32>,
    chapters_label: &str,
    chapters_have_parts: bool,
    empty_prefixes: bool,
    empty_suffixes: bool,
) {
    if (!empty_prefixes || !empty_suffixes) && !chapters_have_parts {
        parsed_content.push_str(&format_part_heading(
            chapters_label,
            headings_auto_identifiers,
            section_number,
        ));
    }
}

/// Remove the comments used to mark the begin and end of each chapter to fix
/// links with more precision. An additional line break between chapters is kept.
fn remove_chapters_marks_comments(parsed_content: &str, source_paths: &[String]) -> String {
    let mut fixed = parsed_content.to_string();
    for source in source_paths {
        let begin_mark = format!("\n<!-- {} begins -->\n\n", source);
        let end_mark = format!("\n<!-- {} ends -->\n", source);

        fixed = fixed.replace(&begin_mark, "\n");
        fixed = fixed.replace(&end_mark, "");
    }

    fixed
}

/// Parse the book contents and return the result of parse all chapters.
#[allow(clippy::too_many_arguments)]
fn parse_book_contents(
    prefix_chapters: &[BookItem],
    content_chapters: &[BookItem],
    suffix_chapters: &[BookItem],
    book_paths: &HashSet<PathBuf>,
    chapters_have_parts: bool,
    initial_level: usize,
    title_labels: &TitleLabels,
    unlist_headings: bool,
    headings_auto_identifiers: bool,
) -> String {
    // The whole document section number is initialized to 0 in order the first
    // heading is 1
    let mut section_number = vec![0];
    let mut source_paths = Vec::new();
    let mut parsed_content = String::new();

    if !prefix_chapters.is_empty() {
        parsed_content.push_str(&format_part_heading(
            &title_labels.preamble,
            headings_auto_identifiers,
            &mut section_number,
        ));
        let (sub_contents, sub_paths) = &recursively_concatenate_book(
            prefix_chapters,
            initial_level,
            book_paths,
            &mut section_number,
            unlist_headings,
            headings_auto_identifiers,
        );
        parsed_content.push_str(sub_contents);
        source_paths.extend_from_slice(sub_paths);
    }

    write_chapters_heading(
        &mut parsed_content,
        headings_auto_identifiers,
        &mut section_number,
        &title_labels.chapters,
        chapters_have_parts,
        prefix_chapters.is_empty(),
        suffix_chapters.is_empty(),
    );
    let (sub_contents, sub_paths) = &recursively_concatenate_book(
        content_chapters,
        initial_level,
        book_paths,
        &mut section_number,
        unlist_headings,
        headings_auto_identifiers,
    );
    parsed_content.push_str(sub_contents);
    source_paths.extend_from_slice(sub_paths);

    if !suffix_chapters.is_empty() {
        parsed_content.push_str(&format_part_heading(
            &title_labels.annexes,
            headings_auto_identifiers,
            &mut section_number,
        ));
        let (sub_contents, sub_paths) = &recursively_concatenate_book(
            suffix_chapters,
            initial_level,
            book_paths,
            &mut section_number,
            unlist_headings,
            headings_auto_identifiers,
        );
        parsed_content.push_str(sub_contents);
        source_paths.extend_from_slice(sub_paths);
    }

    parsed_content = fix_external_links(&parsed_content);

    parsed_content = remove_chapters_marks_comments(&parsed_content, &source_paths);

    parsed_content
}

/// Parse a full mdBook getting (and adding) parts and concatenating prefixes,
/// numbered chapters and suffixes, returning the result as String. writing the contents in the file
/// `book/pandoc/md/book.md` and returning that path.
pub fn parse_book(
    ctx: &RenderContext,
    title_labels: &TitleLabels,
    unlist_headings: bool,
    headings_auto_identifiers: bool,
) -> String {
    // Set the current working directory to the `src` path (everything is relative to the summary)
    let src_path = ctx.root.join("src");
    set_current_dir(&src_path).expect("Error setting the current dir to root path");

    // `BookItems` inner reference is problematic and `.sections` return all sections
    // instead of only the first-depth ones, so this is convenient
    let book_items: Vec<BookItem> = ctx.book.iter().cloned().collect();
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
        unlist_headings,
        headings_auto_identifiers,
    )
}
