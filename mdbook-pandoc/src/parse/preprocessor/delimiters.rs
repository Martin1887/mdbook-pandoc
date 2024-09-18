use regex::{Regex, RegexBuilder};

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
        .replace_all(formatted, "$1")
        .to_string()
}
