use std::path::Path;

use super::PATH_RE;
use anyhow::{Context, Result};
use log::warn;
use regex::{Captures, Regex, RegexBuilder, Replacer};

/// Struct with the text where search the header identifier of the link.
struct HeaderLinkReplacer {
    text: String,
    external_links: bool,
}

/// Filter the text between comments marking start and end of the MD file
/// avoiding boilerplate errors and options handling.
fn filter_text_where_search(text: &str, search_path: &Path) -> Result<String> {
    // Only the file name is searched because the path can be relative
    let file_name = search_path
        .file_name()
        .context("Empty file name")?
        .to_str()
        .context("Error converting file name to str")?;
    let init_pos = text
        .find(&format!("{} begins -->\n", file_name))
        .context("No init pos found")?;
    let end_pos = text[init_pos..text.len()]
        .find(&format!("{} ends -->\n", file_name))
        .context("No end pos found")?;
    Ok(text[init_pos..init_pos + end_pos].to_string())
}

impl Replacer for HeaderLinkReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let mut replacement = String::from(caps.get(0).unwrap().as_str());

        // get the match (the path of a reference or a link)
        let path_match = caps.name("ref_path").unwrap_or_else(|| {
            caps.name("path")
                .expect("No capture with name `ref_path` nor `path` in the regex")
        });
        let path_str = path_match.as_str();

        // internal links must begin by `#`, external links must contain `#` but
        // after a path
        if (self.external_links && path_str.contains('#') && !path_str.starts_with('#'))
            || (!self.external_links && path_str.starts_with('#'))
        {
            let fragment_pos = path_str.rfind('#').unwrap();
            let filtered_text;
            let text_where_search = if self.external_links {
                let search_path = Path::new(&path_str[0..fragment_pos]);
                filtered_text =
                    filter_text_where_search(&self.text, search_path).unwrap_or(String::new());
                &filtered_text
            } else {
                &self.text
            };
            let header_id = if text_where_search.is_empty() {
                None
            } else if fragment_pos + 1 == path_str.len() {
                find_header_id_in_text("", text_where_search, true)
            } else {
                // The link without the `#`
                find_header_id_in_text(
                    &path_str[fragment_pos + 1..path_str.len()],
                    text_where_search,
                    false,
                )
            };

            match header_id {
                Some(id) => replacement = replacement.replace(path_str, &id),
                // The link is not modified, it is dead and a warning is printed
                None => {
                    warn!(
                        "Missing internal link: `{}`. \
                        It may not be a header after merging documents or a wrong reference",
                        path_str
                    );
                }
            }
        }

        dst.push_str(&replacement);
    }
}

/// Fix links relative to the current file setting the header identifier.
pub(crate) fn fix_internal_links(text: &str) -> String {
    fix_links(text, false)
}

/// Fix links relative to another file of the book setting the header identifier.
pub(crate) fn fix_external_links(text: &str) -> String {
    fix_links(text, true)
}

/// Fix links relative to the own file (no file name in the link) or another
/// file of the book according to the `external` argument.
fn fix_links(text: &str, external: bool) -> String {
    let replacer = HeaderLinkReplacer {
        text: text.to_string(),
        external_links: external,
    };
    PATH_RE.replace_all(text, replacer).to_string()
}

/// Find the id of the first matching header including the `#` character in
/// the returned value.
/// Since Markdown links only include
/// the header name, the first possible header match in the text is chosen.
/// So, if several headers contain the same name in the same file, the first
/// one will be chosen. Headers with level greater than 6 do not exist, so
/// links to those headers (in the original MD file their level is lower) are
/// removed and this function returns `None` if it does not find the header.
/// If the `first_header` parameter is `true`, the first header is returned
/// instead of searching.
pub(crate) fn find_header_id_in_text(
    header: &str,
    text: &str,
    first_header: bool,
) -> Option<String> {
    lazy_static! {
        // No spaces nor `}` after the `#` inside the `{}` after the header
        static ref HEADER_ID_RE: Regex = RegexBuilder::new(
            r#"^\s{0,3} \#{1,6}.+ \s* [{]
                (?P<headerid> \# [^\s } ]+ )
                .*
                [}] \s* $"#
        )
        .multi_line(true)
        .dot_matches_new_line(false)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    }
    let mut header_id = None;
    for caps in HEADER_ID_RE.captures_iter(text) {
        // Remove the `#` and the section number of the headerid
        // (if no custom header) comparing only the kebab-case
        if let Some(id) = caps["headerid"][1..].split("__").last() {
            if first_header || id == header {
                header_id = Some(caps["headerid"].to_string());
                break;
            }
        }
    }

    header_id
}
