use pulldown_cmark::HeadingLevel;
use regex::Regex;

use super::HeadingAttrs;

/// Label appended to headings to add the classes `.unnumbered` and `.unlisted`
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

/// Return the new level for the heading adding the current level
/// and the hierarchy level.
pub(crate) fn new_heading_level(level: HeadingLevel, hierarchy_level: usize) -> usize {
    let current_level = heading_level_to_number(level);

    current_level as usize + hierarchy_level
}

/// Return the transformed heading modifying the actual level using the
/// hierarchy level of the MD (or bold text if greater than 6), setting
/// automatic identifier and, only if not the first heading and enabled, adding
/// unlisted and unnumbered classes.
///
/// # Parameters
///
/// - `heading`: The attributes of the heading.
/// - `hierarchy_level`: The hierarchy level to be added to the current level heading.
/// - `first_transform`: `bool` to not remove from the table of contents the first heading.
/// - `unlist_headings`: If unlisted and unnumbered classes must be added to not first headings.
/// - `section_number`: All the section numbers. Unnumbered headings are also counted and
///   the section number is modified here in function of the level of the current headings.
///
/// # Returns
///
/// The heading String.
pub(crate) fn transform_heading(
    heading: &HeadingAttrs,
    hierarchy_level: usize,
    first_transform: bool,
    unlist_headings: bool,
    headings_auto_identifiers: bool,
    section_number: &mut Vec<u32>,
) -> String {
    let new_heading_level = new_heading_level(heading.level, hierarchy_level);
    // Add a dot prefix to classes
    let mut new_attributes: Vec<String> =
        heading.classes.iter().map(|c| format!(".{}", c)).collect();
    new_attributes.append(
        &mut heading
            .attrs
            .iter()
            .map(|(a, v)| format!("{}={}", a, v.as_ref().unwrap_or(&"none".to_string())))
            .collect::<Vec<String>>(),
    );
    update_section_number(section_number, new_heading_level);
    // Markdown only supports 6 levels, so following levels are coded as
    // simply bold text
    if new_heading_level <= 6 {
        // The first transformation does not remove the numeration because
        // it is the section title
        if !first_transform && unlist_headings {
            new_attributes.push(UNNUMBERED.to_string());
            new_attributes.push(UNLISTED.to_string());
        };
        // The identifier must be the first attribute
        if heading.id.is_some() || headings_auto_identifiers {
            new_attributes.insert(
                0,
                format!(
                    "#{}",
                    heading
                        .id
                        .clone()
                        .unwrap_or_else(|| heading_identifier(&heading.text, section_number),),
                ),
            );
        }
        let heading_attributes_str = if new_attributes.is_empty() {
            "".to_string()
        } else {
            format!(" {{{}}}", new_attributes.join(" "))
        };
        format!(
            "{} {}{}\n",
            "#".repeat(new_heading_level),
            heading.text,
            heading_attributes_str,
        )
    } else {
        // Bold text
        format!("\n**{}**\n\n", heading.text)
    }
}

/// Update the section number in place in function of the last section number
/// and the level of the current heading.
pub(crate) fn update_section_number(section_number: &mut Vec<u32>, heading_level: usize) {
    let section_len = section_number.len();
    if section_len != heading_level {
        // Remove the last numbers until reach the level or add 1s until that
        section_number.resize(heading_level, 1);
    }
    // The last number is only increased when the level is not greater
    if heading_level <= section_len {
        // Increment the last number
        let new_last_section_number = section_number.pop().unwrap() + 1;
        section_number.push(new_last_section_number);
    }
}

/// Provide an unique identifier for the heading based on full hierarchy and text
/// in kebab-case. Unnumbered headings are also counted, so the identifier number
/// differs from the actual number if there are unnumbered headings.
pub(crate) fn heading_identifier(heading_text: &str, section_number: &[u32]) -> String {
    // Concatenate each number of the section and the heading name with '__'
    format!(
        "h{}{}",
        section_number
            .iter()
            .fold(String::new(), |acc, x| format!("{}{}__", acc, x),),
        heading_identifier_sanitize(heading_text)
    )
}

/// Convert a text into kebab-case.
pub(crate) fn heading_identifier_sanitize(text: &str) -> String {
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
