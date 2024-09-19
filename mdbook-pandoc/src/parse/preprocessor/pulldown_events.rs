use std::ops::Range;

use pulldown_cmark::{CodeBlockKind, Event::*, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Store the needed heading attributes to modify headings.
#[derive(Debug)]
pub(crate) struct HeadingAttrs {
    pub(crate) id: Option<String>,
    pub(crate) level: HeadingLevel,
    pub(crate) text: String,
    pub(crate) classes: Vec<String>,
    pub(crate) attrs: Vec<(String, Option<String>)>,
}

/// Return all the parsed headings and their position to replace them in the str.
pub(crate) fn get_headings(md: &str) -> Vec<(Range<usize>, HeadingAttrs)> {
    let mut headings = Vec::new();
    let mut parsing_heading = false;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = Parser::new_ext(md, options).into_offset_iter();

    for (event, range) in parser {
        match event {
            Start(tag) => start_heading_tag(tag, range, &mut headings, &mut parsing_heading),
            End(tag) => end_heading_tag(tag, range, &mut headings, &mut parsing_heading),
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

fn start_heading_tag(
    tag: Tag,
    range: Range<usize>,
    headings: &mut Vec<(Range<usize>, HeadingAttrs)>,
    parsing_heading: &mut bool,
) {
    if let Tag::Heading {
        level,
        id,
        classes,
        attrs,
    } = tag
    {
        let new_heading = HeadingAttrs {
            id: id.map(|s| s.to_string()),
            level,
            text: String::new(),
            classes: classes.iter().map(|c| c.to_string()).collect(),
            attrs: attrs
                .iter()
                .map(|(a, v)| (a.to_string(), v.as_ref().map(|v| v.to_string())))
                .collect(),
        };
        *parsing_heading = true;
        headings.push((range, new_heading));
    }
}

fn end_heading_tag(
    tag: TagEnd,
    range: Range<usize>,
    headings: &mut [(Range<usize>, HeadingAttrs)],
    parsing_heading: &mut bool,
) {
    if let TagEnd::Heading(_level) = tag {
        // heading attributes are parsed at start, here only the end of the
        // range is changed
        let last = headings.last_mut().unwrap();

        last.0 = Range {
            start: last.0.start,
            end: range.end,
        };
        *parsing_heading = false;
    }
}

/// Store the needed heading attributes to modify headings.
#[derive(Debug)]
pub(crate) struct FencedCodeblock {
    /// The attrs that must be removed.
    /// They are not an `Option` because for no custom attrs codeblocks objects
    /// of this struct are not created.
    pub(crate) custom_attrs: String,
}

/// Remove custom fenced code blocks attributes, since they are not supported
/// by Pandoc.
pub(crate) fn get_fenced_code_blocks_with_custom_attrs(
    md: &str,
) -> Vec<(Range<usize>, FencedCodeblock)> {
    let mut fenced_codeblocks = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = Parser::new_ext(md, options).into_offset_iter();

    for (event, range) in parser {
        if let Start(Tag::CodeBlock(CodeBlockKind::Fenced(attrs))) = event {
            // only handle normal attrs, not the enclosed by curly
            // braces used by Pandoc
            if !attrs.is_empty() && !attrs.starts_with("{") && attrs.contains(",") {
                let comma = attrs.find(',').unwrap();
                fenced_codeblocks.push((
                    Range {
                        start: range.start,
                        end: range.end,
                    },
                    FencedCodeblock {
                        custom_attrs: attrs[comma + 1..attrs.len()].to_string(),
                    },
                ));
            }
        }
    }

    fenced_codeblocks
}
