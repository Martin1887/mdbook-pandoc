use regex::{Regex, RegexBuilder};

/// Replace the math delimiters of mdBook by the pandoc ones
pub(crate) fn replace_math_delimiters(text: &str) -> String {
    lazy_static! {
        // \\( and \\) with any number of spaces
        static ref INLINE_MATH_RE: Regex = Regex::new(
            r"(\\{2}\(\s*|\s*\\{2}\))"
        ).unwrap();
        // \\[ and \\] with any number of spaces
        static ref DISPLAY_MATH_RE: Regex = Regex::new(
            r"(\\{2}\[\s*|\s*\\{2}\])").unwrap();
    }
    let mut formatted = INLINE_MATH_RE.replace_all(text, "$").to_string();
    // `Replacer` uses 4 `$` to put only 2 `$` because `$` is used for capture groups
    formatted = DISPLAY_MATH_RE.replace_all(&formatted, "$$$$").to_string();

    formatted
}

/// Replace the code annotations starting with comma, because they are mdBook
/// exclusive and they don't work on Pandoc breaking the language
pub(crate) fn replace_custom_mdbook_code_block_annotations(formatted: &str) -> String {
    lazy_static! {
        // capture the block specification until the first comma
        static ref CUSTOM_CODE_BLOCK_ANNOTATIONS_RE: Regex = RegexBuilder::new(
            r"(^\s*[`]{3}\w*),.*\s*$"
        ).multi_line(true).dot_matches_new_line(false).build().unwrap();
    }
    CUSTOM_CODE_BLOCK_ANNOTATIONS_RE
        .replace_all(&formatted, "$1")
        .to_string()
}
