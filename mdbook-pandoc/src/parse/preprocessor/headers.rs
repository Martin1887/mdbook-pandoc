use regex::Regex;

/// Label appended to headers to add the classes `.unnumbered` and `.unlisted`
pub(crate) const UNNUMBERED_UNLISTED: &'static str = " .unnumbered .unlisted";

/// Type of MD header: Atx (#), Setext1 (====) or Setext2(----).
pub(crate) enum HeaderType {
    Atx,
    Setext1,
    Setext2,
}

/// Return the header type of the line or `None` if it is not a header.
pub(crate) fn header_type(line: &str, next_line: &str) -> Option<HeaderType> {
    lazy_static! {
        // 0-3 spaces before the first '#', at most 6 '#' and whitespace or end
        // of line after the last '#'
        static ref ATX_RE: Regex = Regex::new(
            r"^[ ]{0,3}#{1,6}(\s|$).*$?"
        ).unwrap();

        static ref SETEXT_RE_STR: &'static str = r"^[ ]{0,3}<CHAR>+\s*$";
        static ref SETEXT1_RE: Regex = Regex::new(&SETEXT_RE_STR.replace("<CHAR>", "=")).unwrap();
        static ref SETEXT2_RE: Regex = Regex::new(&SETEXT_RE_STR.replace("<CHAR>", "-")).unwrap();
    }
    if ATX_RE.is_match(&line) {
        Some(HeaderType::Atx)
    } else if SETEXT1_RE.is_match(&next_line) {
        Some(HeaderType::Setext1)
    } else if SETEXT2_RE.is_match(&next_line) {
        Some(HeaderType::Setext2)
    } else {
        None
    }
}

/// Return the new level for the header adding the current level
/// and the hierarchy level.
pub(crate) fn new_header_level(
    line: &str,
    hierarchy_level: usize,
    header_type: HeaderType,
) -> usize {
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
///
/// - `line`: The line of the header.
/// - `next_line`: The next line to check Setext headers.
/// - `hierarchy_level`: The hierarchy level to be added to the current level header.
/// - `first_transform`: `bool` to not remove from the table of contents the first header.
/// - `section_number`: All the section numbers. Unnumbered headers are also counted and
/// the section number is modified here in function of the level of the current header.
///
/// # Returns
///
/// The modified line, if the next line must be skipped and if the line has been
/// transformed.
pub(crate) fn transform_header<'a>(
    line: &str,
    next_line: &'a str,
    hierarchy_level: usize,
    first_transform: bool,
    mut section_number: &mut Vec<u32>,
) -> (String, bool, bool) {
    match header_type(&line, &next_line) {
        None => {
            // Unmodified lines
            (String::from(line), false, false)
        }
        Some(header_type) => {
            let skip_next_line = match header_type {
                HeaderType::Atx => false,
                _ => true,
            };
            let new_header_level = new_header_level(&line, hierarchy_level, header_type);
            update_section_number(&mut section_number, new_header_level);
            let clean_line = String::from(line.replace("#", "").trim());
            let transformed_line = if new_header_level > 6 {
                // Markdown only supports 6 levels, so following levels are coded as
                // simply bold text
                format!("**{}**\n", clean_line)
            } else {
                // The first transformation does not remove the numeration because
                // it is the section title
                format!(
                    "{} {} {{#{}{}}}",
                    "#".repeat(new_header_level),
                    clean_line,
                    header_identifier(&clean_line, section_number),
                    if first_transform {
                        ""
                    } else {
                        UNNUMBERED_UNLISTED
                    }
                )
            };

            (transformed_line, skip_next_line, true)
        }
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
    NOT_WORDS_NOR_NUMBERS
        .replace_all(&text.to_lowercase(), "-")
        .to_string()
}
