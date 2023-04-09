/// Fix references to headings of the same MD or to another MD of the book.
mod internal_references;
/// Fix relative paths.
mod paths;

/// Check that links to references are valid to show warnings if they are not.
mod links_to_references;

#[cfg(test)]
mod tests;

pub(crate) use internal_references::*;
pub(crate) use links_to_references::*;
pub(crate) use paths::*;

use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref URI_PROTOCOL_RE: Regex = Regex::new(r"^[a-wA-W0-9]+://.+$").unwrap();
    static ref PATH_PUNCTUATION: String = r#" :./\\ \-_?!;,{}\[\]+* '"@\$~%= "#.to_string();
    static ref SPACES_WITHOUT_LINE_BREAKS: String = r"[ \  \t ]*".to_string();
    static ref SPACES_WITH_OPTIONAL_AT_MOST_ONE_LINE_BREAK: String =
        format!(r"{blanks} (\n)? {blanks}", blanks=*SPACES_WITHOUT_LINE_BREAKS);

    // Catch links references and links. Matching `<`, `>`, quotes and parenthesis
    // are not checked (that requires a stack) for performance reasons, as
    // the purpose is only to translate paths but not correct Markdown parsing.
    // The spaces inside path only if between `<` and `>` and only complex
    // rules are not checked to make the regex simpler because the same reasons.
    //
    // Links references: label inside square brackets followed by a colon
    // and blank characters including at most one line break.
    //
    // Links: square brackets (preceded by exclamation mark if images) and
    // the link inside parenthesis, that can contain spaces only if is
    // between angle brackets
    //
    // Links to references: like links but with a reference label inside square
    // brackets instead of the link inside parenthesis.
    static ref LINK_DESTINATION_STR: String = format!(
        r"{blanks} <? (?P<path> ( [\w\ \t \# {punct} ] )* [\#\w] ) >? {blanks}",
        punct=*PATH_PUNCTUATION, blanks=*SPACES_WITHOUT_LINE_BREAKS
    );
    static ref LINK_DESTINATION_REF_STR: String =
        LINK_DESTINATION_STR.replace("path", "ref_path");
    static ref LINK_TEXT_STR: String = r#"( [^ \] \n ]+ )*"#.to_string();
    static ref LINK_TITLE_STR: String =
        r#"["'(] ( [^)'" \n ]+ (\n)? )* ["')]"#.to_string();

    static ref REF_RE_STR: String = format!(
        r#"^ [\ ]{{0,3}} \[ (?P<label>{text}) \] : {sep} {ref_dest} ( {sep} {title} )? $"#,
        text=*LINK_TEXT_STR,
        ref_dest=*LINK_DESTINATION_REF_STR,
        title=*LINK_TITLE_STR,
        sep=*SPACES_WITH_OPTIONAL_AT_MOST_ONE_LINE_BREAK
    );
    static ref REFERENCE_LINK_RE: Regex = RegexBuilder::new(&REF_RE_STR)
        .multi_line(true)
        .dot_matches_new_line(false)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    // A backslash before the first square bracket avoids the link.
    static ref LINK_RE_STR: String = format!(
        r#"( ^ | [^\\] ) !? \[ {text} \] \( {dest} ( {sep} {title} )? \) "#,
        text=*LINK_TEXT_STR,
        dest=*LINK_DESTINATION_STR,
        title=*LINK_TITLE_STR,
        sep=*SPACES_WITH_OPTIONAL_AT_MOST_ONE_LINE_BREAK
    );

    static ref PATH_RE_STR: String = format!(
        r#"({ref_link} | {link})"#,
        ref_link=*REF_RE_STR,
        link=*LINK_RE_STR
    );
    static ref PATH_RE: Regex = RegexBuilder::new(&PATH_RE_STR)
        .multi_line(true)
        .dot_matches_new_line(false)
        .ignore_whitespace(true)
        .build()
        .unwrap();

    // The text is omitted in collapsed links (empty square brackets)
    // and the second square brackets are omitted in shortcut links.
    // As in the normal links, a backslash before the first square bracket
    // avoids the link.
    // Actual links are avoided requiring a not `(` after the first block.
    static ref LINK_TO_REF_RE_STR: String = format!(
        r#"( ^ | [^\\] ) \[ (?P<text>{text}) \] ( \[ (?P<label>{text})? \] | [^()] )"#,
        text=*LINK_TEXT_STR
    );
    static ref LINK_TO_REF_RE: Regex = RegexBuilder::new(&LINK_TO_REF_RE_STR)
        .multi_line(true)
        .dot_matches_new_line(false)
        .ignore_whitespace(true)
        .build()
        .unwrap();
}
