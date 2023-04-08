use pulldown_cmark::HeadingLevel;
use regex::Regex;

use super::HeadingAttrs;

/// Label appended to headers to add the classes `.unnumbered` and `.unlisted`
pub(crate) const UNNUMBERED: &str = ".unnumbered";
pub(crate) const UNLISTED: &str = ".unlisted";

/// Return the equivalent number (1-6) of the heading level.
pub(crate) fn heading_level_to_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Return the new level for the header adding the current level
/// and the hierarchy level.
pub(crate) fn new_header_level(level: HeadingLevel, hierarchy_level: usize) -> usize {
    let current_level = heading_level_to_number(level);

    current_level as usize + hierarchy_level
}

/// Return the transformed header modifying the actual level using the
/// hierarchy level of the MD (or bold text if greater than 6), setting
/// automatic identifier and, only if not the first header and enabled, adding
/// unlisted and unnumbered classes.
///
/// # Parameters
///
/// - `heading`: The attributes of the heading.
/// - `hierarchy_level`: The hierarchy level to be added to the current level header.
/// - `first_transform`: `bool` to not remove from the table of contents the first header.
/// - `unlist_headers`: If unlisted and unnumbered classes must be added to not first headers.
/// - `section_number`: All the section numbers. Unnumbered headers are also counted and
/// the section number is modified here in function of the level of the current header.
///
/// # Returns
///
/// The heading String.
pub(crate) fn transform_header(
    heading: &HeadingAttrs,
    hierarchy_level: usize,
    first_transform: bool,
    unlist_headers: bool,
    section_number: &mut Vec<u32>,
) -> String {
    let new_heading_level = new_header_level(heading.level, hierarchy_level);
    // Add a dot prefix to classes
    let mut new_attributes: Vec<String> =
        heading.classes.iter().map(|c| format!(".{}", c)).collect();
    update_section_number(section_number, new_heading_level);
    // Markdown only supports 6 levels, so following levels are coded as
    // simply bold text
    if new_heading_level <= 6 {
        // The first transformation does not remove the numeration because
        // it is the section title
        if !first_transform && unlist_headers {
            new_attributes.push(UNNUMBERED.to_string());
            new_attributes.push(UNLISTED.to_string());
        };
        // The identifier must be the first attribute
        // TODO: use the id if it exists (requires changes in other parts of the code)
        new_attributes.insert(
            0,
            format!("#{}", header_identifier(&heading.text, section_number)),
        );
        format!(
            "{} {} {{{}}}\n",
            "#".repeat(new_heading_level),
            heading.text,
            new_attributes.join(" ")
        )
    } else {
        // Bold text
        format!("\n**{}**\n\n", heading.text)
    }
}

/// Update the section number in place in function of the last section number
/// and the level of the current header.
pub(crate) fn update_section_number(section_number: &mut Vec<u32>, header_level: usize) {
    let section_len = section_number.len();
    if section_len != header_level {
        // Remove the last numbers until reach the level or add 1s until that
        section_number.resize(header_level, 1);
    }
    // The last number is only increased when the level is not greater
    if header_level <= section_len {
        // Increment the last number
        let new_last_section_number = section_number.pop().unwrap() + 1;
        section_number.push(new_last_section_number);
    }
}

/// Provide an unique identifier for the header based on full hierarchy and text
/// in kebab-case. Unnumbered headers are also counted, so the identifier number
/// differs from the actual number if there are unnumbered headers.
pub(crate) fn header_identifier(header_text: &str, section_number: &[u32]) -> String {
    // Concatenate each number of the section and the header name with '__'
    format!(
        "h{}{}",
        section_number
            .iter()
            .fold(String::new(), |acc, x| format!("{}{}__", acc, x),),
        header_identifier_sanitize(header_text)
    )
}

/// Convert a text into kebab-case.
pub(crate) fn header_identifier_sanitize(text: &str) -> String {
    lazy_static! {
        // The `+` quantifier is needed because contiguous characters must be
        // changed by only one
        static ref NOT_WORDS_NOR_NUMBERS: Regex = Regex::new(r"[_[^\w\d]]+").unwrap();
    }
    // Non ASCII characters must be escaped and they are transformed in
    // `\things` so those slashes must also be converted to dashes
    NOT_WORDS_NOR_NUMBERS
        .replace_all(&text.to_lowercase(), "-")
        .as_bytes()
        .escape_ascii()
        .to_string()
        .replace('\\', r"-")
}
