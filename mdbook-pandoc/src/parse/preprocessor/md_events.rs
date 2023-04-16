use std::ops::Range;

use pulldown_cmark::{Event::*, HeadingLevel, Options, Parser, Tag};
use regex::{Regex, RegexBuilder};

/// Store the needed heading attributes to modify headings.
#[derive(Debug)]
pub(crate) struct HeadingAttrs {
    pub(crate) id: Option<String>,
    pub(crate) level: HeadingLevel,
    pub(crate) text: String,
    pub(crate) classes: Vec<String>,
}

/// Return all the parsed headings and their position to replace them in the str.
pub(crate) fn get_headings(md: &str) -> Vec<(Range<usize>, HeadingAttrs)> {
    lazy_static! {
        static ref METADATA_RE: Regex = RegexBuilder::new(r"(^\n?|\n\n)(---\n)(.+?)\n---($|\n)")
            .multi_line(false)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
    }
    let mut headings = Vec::new();
    let mut parsing_heading = false;
    // Metadata headers must be modified replacing the end delimiter to
    // `...` instead of `---` because pulldown_cmark does not support metadata
    // blocks and handle them as h2 setext headings
    let metadata_blocks_fixed = METADATA_RE.replace_all(md, format!("$1$2$3\n...$4"));
    let parser = Parser::new_ext(&metadata_blocks_fixed, Options::ENABLE_HEADING_ATTRIBUTES)
        .into_offset_iter();

    for (event, range) in parser {
        match event {
            Start(tag) => start_tag(tag, range, &mut headings, &mut parsing_heading),
            End(tag) => end_tag(tag, range, &mut headings, &mut parsing_heading),
            Text(text) => {
                if parsing_heading {
                    let last = headings.last_mut().unwrap();
                    if !last.1.text.is_empty() {
                        // each line of the heading is passed in a different
                        // event, so line breaks are replaced by spaces
                        last.1.text.push(' ');
                    }
                    last.1.text.push_str(text.as_ref());
                }
            }
            _ => {}
        }
    }

    headings
}

fn start_tag(
    tag: Tag,
    range: Range<usize>,
    headings: &mut Vec<(Range<usize>, HeadingAttrs)>,
    parsing_heading: &mut bool,
) {
    if let Tag::Heading(level, id, classes) = tag {
        let new_heading = HeadingAttrs {
            id: id.map(|s| s.to_string()),
            level,
            text: String::new(),
            classes: classes.iter().map(|c| c.to_string()).collect(),
        };
        *parsing_heading = true;
        headings.push((range, new_heading));
    }
}

fn end_tag(
    tag: Tag,
    range: Range<usize>,
    headings: &mut [(Range<usize>, HeadingAttrs)],
    parsing_heading: &mut bool,
) {
    if let Tag::Heading(_level, _id, _classes) = tag {
        // heading attributes are parsed at start, here only the end of the
        // range is changed
        let mut last = headings.last_mut().unwrap();

        last.0 = Range {
            start: last.0.start,
            end: range.end,
        };
        *parsing_heading = false;
    }
}
