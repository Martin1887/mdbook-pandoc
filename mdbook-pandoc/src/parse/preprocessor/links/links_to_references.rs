use log::warn;
use regex::Regex;
use unicase::UniCase;

use super::{LINK_TO_REF_RE, REFERENCE_LINK_RE};

/// Normalize a label without the square brackets to match it: perform
/// the Unicode case fold, strip leading and trailing blank characters and
/// collapse consecutive internal spaces, tabs and line endings to a single
/// space.
///
/// # Parameters:
///
/// - `label`: The label without the square brackets.
///
/// # Returns:
///
/// The normalized label.
pub(crate) fn normalize_label(label: &str) -> UniCase<String> {
    lazy_static! {
        static ref CONSECUTIVE_SPACES_RE: Regex = Regex::new(r"\s+").unwrap();
    }
    UniCase::new(
        CONSECUTIVE_SPACES_RE
            .replace_all(&String::from(label.trim()), " ")
            .to_string(),
    )
}

/// Return `true` if a reference with the same label is found and `false`
/// otherwise.
pub(crate) fn label_matches(label: &str, text: &str) -> bool {
    let mut matched = false;

    let normalized_label = normalize_label(label);
    for capt in REFERENCE_LINK_RE.captures_iter(text) {
        let normalized_capture = normalize_label(&capt["label"]);
        if normalized_capture == normalized_label {
            matched = true;
            break;
        }
    }

    matched
}

/// Log a warning for each link to reference that does not match a reference link.
pub(crate) fn check_ref_links(text: &str) {
    for capt in LINK_TO_REF_RE.captures_iter(text) {
        // if there is no label (shortcut link) the label is the text
        let mut label = capt
            .name("label")
            .unwrap_or(capt.name("text").unwrap())
            .as_str();
        // `label` may be empty (collapsed link) and then `text` must be used
        if label.is_empty() {
            label = &capt["text"];
        }
        if !label_matches(label, text) {
            warn!("Reference link not found for the label `{label}`");
        }
    }
}
