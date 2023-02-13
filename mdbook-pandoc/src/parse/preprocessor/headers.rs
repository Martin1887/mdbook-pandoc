use regex::{Captures, Regex, RegexBuilder};

/// Label appended to headers to add the classes `.unnumbered` and `.unlisted`
pub(crate) const UNNUMBERED_UNLISTED: &'static str = " .unnumbered .unlisted";

/// Type of MD header: Atx (#), Setext1 (====) or Setext2(----).
#[allow(dead_code)]
pub(crate) enum HeaderType {
    Atx,
    Setext1,
    Setext2,
}

/// Return `true` if the line is a ATX header and `false` otherwise.
pub(crate) fn is_atx_header(line: &str) -> bool {
    lazy_static! {
        // 0-3 spaces before the first '#', at most 6 '#' and whitespace or end
        // of line after the last '#'
        static ref ATX_RE: Regex = Regex::new(
            r"^[ ]{0,3}#{1,6}(\s|$).*$?"
        ).unwrap();
    }
    ATX_RE.is_match(&line)
}

/// Custom replacer needed to write one or two `#` in function of the SETEXT
/// header (`=` or `-`) and to check that the prelude is not a paragraph.
fn setext_replacer(caps: &Captures) -> String {
    lazy_static! {
        // A paragraph cannot have more than 3 initial spaces nor starting by
        // `>` and must have at least one letter or number.
        static ref PARAGRAPH_RE: Regex = RegexBuilder::new(r#"^[ ]{0,3}[^>]+.*[\w\d].*$"#)
            .multi_line(true)
            .dot_matches_new_line(false)
            .build()
            .unwrap();
    }

    let prelude = match caps.name("prelude") {
        Some(s) => s.as_str(),
        None => "",
    };

    // replace nothing if the prelude is a paragraph or 3 dashes (YAML header)
    let new_header: String = if PARAGRAPH_RE.is_match(&prelude) || prelude.trim() == "---" {
        format!("{}{}", &caps["header"], &caps["underline"])
    } else {
        let formatted_header = caps["header"]
            .trim()
            .replace("\r\n", " ")
            .replace("\r", " ")
            .replace("\n", " ");
        match &caps["underline"]
            .chars()
            .filter(|c| *c == '=' || *c == '-')
            .next()
        {
            Some('=') => format!("# {}\n", &formatted_header),
            Some('-') => format!("## {}\n", &formatted_header),
            _ => panic!("Wrong underline in SETEXT replacer"),
        }
    };

    format!("{}{}", prelude, new_header)
}

/// Replace SETEXT headers by the equivalent ATX ones in the whole text.
pub(crate) fn setext2atx(md: &str) -> String {
    lazy_static! {
        // A SETEXT can have several lines and prelude must by anything except a
        // paragraph. As negative lookaheads are not supported, the paragraph
        // exception is checked in the replacer.
        // Prelude is optional because the header can be at the first line.
        // Line breaks are necessary in the middle of regex and all line endings
        // may happen here because the MD text is not transformed yet.
        static ref LINE_BREAK_RE_STR: &'static str = r"( \n | \r | \r\n )";
        static ref ANY_LINE_RE_STR: &'static str = r".*";
        static ref SETEXT_HEADER_RE_STR: String = format!(
            r"[\ ]{{0,3}} (. {}?)+", *LINE_BREAK_RE_STR
        );
        static ref SETEXT_UNDERLINE_RE_STR: &'static str =
            r"[\ ]{0,3} ( (=)+ | (-)+ ) [\ \t]*";
        static ref SETEXT_RE_STR: String = format!(
            r"^
            (?P<prelude> {line} {br})?
            (?P<header> {header} {br})
            (?P<underline> {underline})
            $",
            line=*ANY_LINE_RE_STR,
            br=*LINE_BREAK_RE_STR,
            header=*SETEXT_HEADER_RE_STR,
            underline=*SETEXT_UNDERLINE_RE_STR
        );
        static ref SETEXT_RE: Regex = RegexBuilder::new(&SETEXT_RE_STR)
            .multi_line(true)
            .dot_matches_new_line(false)
            .ignore_whitespace(true)
            .build()
            .unwrap();
    }

    SETEXT_RE.replace_all(md, setext_replacer).to_string()
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

/// Return the transformed `line` analyzing header patterns.
/// If the `line` is a header, it is returned changing its level and appending
/// id and '.unnumbered' and '.unlisted' classes to remove numbers and entries
/// from TOC. Otherwise the same `String` is returned.
///
/// # Parameters
///
/// - `line`: The line of the header.
/// - `hierarchy_level`: The hierarchy level to be added to the current level header.
/// - `first_transform`: `bool` to not remove from the table of contents the first header.
/// - `section_number`: All the section numbers. Unnumbered headers are also counted and
/// the section number is modified here in function of the level of the current header.
///
/// # Returns
///
/// The modified line and if the line has been transformed.
pub(crate) fn transform_header<'a>(
    line: &str,
    hierarchy_level: usize,
    first_transform: bool,
    mut section_number: &mut Vec<u32>,
) -> (String, bool) {
    if is_atx_header(&line) {
        let new_header_level = new_header_level(&line, hierarchy_level, HeaderType::Atx);
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

        (transformed_line, true)
    } else {
        // Unmodified lines
        (String::from(line), false)
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
        .replace(r"\", r"-")
}
