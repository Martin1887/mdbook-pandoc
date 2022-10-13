//! mdBook backend that is able to generate many output formats using `pandoc`.
//! Note that `pandoc` must be installed in your system. You also need other
//! tools for some formats like PDF.
//!
//! The following opinionated rules are applied to the book, supposing that
//! headers that are not the main header are used only to highlight and
//! separate elements in the page but they must not be in the table of contents
//! nor be numbered (if an actual title is desired its contents should be in a
//! different Markdown file):
//! - If the book have prefixes or suffixes a part is created for each of them.
//! - If parts are created for prefixes and/or suffixes and the numbered
//! chapters don't have parts then a part is created for the chapters, the
//! original parts are used for the numbered chapters otherwise.
//! - The headers of the files are downgraded adding the level of the file
//! (e.g. a chapter inside a part has level 1 so its 1-level header becomes 2-level).
//! - If a header's level becomes bigger than 6 then the text is simply bold.
//! - Setext headers (underlined ones) are changed to atx (hashes prefix).
//! - Headers that are not the main one are labeled with `{.unnumbered .unlisted}`
//! to remove numbers and avoid that they appear in the table of contents.
//! - Each Markdown file must have a main 1st level header as the first header
//! of the file and that header is used as its header, the name of the file is
//! ignored.

mod config;
mod metadata;
mod parse;
#[cfg(test)]
mod tests;

#[macro_use]
extern crate lazy_static;

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use config::metadata::MetadataConfig;
use config::TitleLabels;
use mdbook::{renderer::RenderContext, Renderer};
use parse::parse_book;

pub struct PandocRenderer;
impl Renderer for PandocRenderer {
    fn name(&self) -> &str {
        "pandoc"
    }

    /// Main function that:
    /// - reads configuration
    /// - parse book
    /// - convert book in the specified formats using pandoc.
    fn render(&self, ctx: &RenderContext) -> mdbook::errors::Result<()> {
        // TODO: Read from configs
        let title_labels = TitleLabels {
            preamble: String::from("Preamble"),
            chapters: String::from("Chapters"),
            annexes: String::from("Annexes"),
        };

        let metadata = match ctx
            .config
            .get_deserialized_opt::<MetadataConfig, &str>("output.pandoc.metadata")
            .expect("Error reading the configuration file")
        {
            Some(config_text) => format!(
                "---\n{}---\n",
                match config_text {
                    MetadataConfig::Path(path) =>
                        fs::read_to_string(path).expect("Error reading the YAML path"),
                    MetadataConfig::Metadata(metadata) => {
                        serde_yaml::to_string(&metadata).unwrap()
                    }
                }
            ),
            None => String::new(),
        };

        let parsed = format!("{}{}", metadata, parse_book(&ctx, &title_labels));

        let _parsed_path = write_pandoc_md_file(&ctx.destination, &parsed);
        Ok(())
    }
}

/// Write the parsed contents into the Pandoc MD file and return that path
/// (`./book/pandoc/md/book.md`)
fn write_pandoc_md_file(dest_path: &Path, parsed_content: &str) -> PathBuf {
    let mut md_path = dest_path.to_owned().clone();
    md_path.push("book.md");

    let mut md_out = File::create(&md_path).expect("Error writing the parsed MD file");
    md_out
        .write_all(parsed_content.as_bytes())
        .expect("Error writing the parsed MD File");

    md_path
}
