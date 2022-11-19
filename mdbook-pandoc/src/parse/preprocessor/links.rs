/// Fix references to headers of the same MD or to another MD of the book.
mod internal_references;
/// Fix relative paths.
mod paths;
#[cfg(test)]
mod tests;

pub(crate) use internal_references::*;
pub(crate) use paths::*;

use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATH_PUNCTUATION: String = r#" :./\\ \-_?!;,{}\[\]+* '"@\$~%= "#.to_string();
    static ref SPACES_WITHOUT_LINE_BREAKS: String = r"[ \  \t ]*".to_string();
    static ref SPACES_WITH_OPTIONAL_AT_MOST_LINE_BREAK: String =
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
    static ref LINK_DESTINATION_STR: String = format!(
        r"{blanks} <? (?P<path> ( [\w\ \t \# {punct} ] )* [\#\w] ) >? {blanks}",
        punct=*PATH_PUNCTUATION, blanks=*SPACES_WITHOUT_LINE_BREAKS
    );
    static ref LINK_DESTINATION_REF_STR: String =
        LINK_DESTINATION_STR.replace("path", "ref_path");
    static ref LINK_TEXT_STR: String = r#"( [^ \] \n ]+ )*"#.to_string();
    static ref LINK_TITLE_STR: String =
        r#"["'(] ( [^)'" \n ]+ (\n)? )* ["')]"#.to_string();

    static ref PATH_RE_STR: String = format!(
        r#"(
            # reference link (https://spec.commonmark.org/0.30/#link-reference-definitions)
            ^ [\ ]{{0,3}} \[ {text} \] : {sep} {ref_dest} ( {sep} {title} )? $
            |
            # link (https://spec.commonmark.org/0.30/#links)
            !? \[ {text} \] \( {dest} ( {sep} {title} )? \) 
        )"#,
        ref_dest=*LINK_DESTINATION_REF_STR, dest=*LINK_DESTINATION_STR,
        text=*LINK_TEXT_STR, title=*LINK_TITLE_STR,
        sep=*SPACES_WITH_OPTIONAL_AT_MOST_LINE_BREAK);

    static ref PATH_RE: Regex = RegexBuilder::new(&PATH_RE_STR)
        .multi_line(true)
        .dot_matches_new_line(false)
        .ignore_whitespace(true)
        .build()
        .unwrap();
}
