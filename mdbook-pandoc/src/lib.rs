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

pub mod config;
pub mod parse;
#[cfg(test)]
mod tests;

#[macro_use]
extern crate lazy_static;

use std::{
    env::set_current_dir,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use config::{epub_metadata::EpubMetadata, MdBookPandocConfig};
use env_logger::Env;
use log::warn;
use mdbook::{renderer::RenderContext, Renderer};
use parse::parse_book;

const COMMAND_ERROR_MSG: &str = r#"Error launching a Pandoc command, check that \
Pandoc is installed in your system and available in your path as `pandoc`"#;

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
        let log_result =
            env_logger::Builder::from_env(Env::default().default_filter_or("warn")).try_init();
        if log_result.is_err() {
            warn!("Error initializing the log.");
        }

        let cfg = ctx
            .config
            .get_deserialized_opt::<MdBookPandocConfig, &str>("output.pandoc")
            .expect("Error reading the configuration file")
            .unwrap();
        let general_cfg = cfg.general;
        let title_labels = &general_cfg.labels;
        let unlist_headers: bool = general_cfg.unlist_not_main_headers;

        let parsed = process_metadata_block(
            parse_book(&ctx, &title_labels, unlist_headers),
            &general_cfg.epub_metadata_fields,
        );
        let parsed_path = write_pandoc_md_file(&ctx.destination, &parsed);

        // the path must be the book `src` folder because paths are relative to
        // that directory
        set_current_dir(&ctx.source_dir())?;
        for (extension, pandoc_command) in cfg.commands {
            pandoc_command.write_template_and_assets_files(&general_cfg);
            let mut command =
                pandoc_command.command(&ctx.destination, &parsed_path, &extension, &general_cfg);
            let command_args_str = format!(
                "{:#?}",
                command
                    .get_args()
                    .collect::<Vec<&std::ffi::OsStr>>()
                    .join(std::ffi::OsStr::new(" "))
            );
            log::info!(
                "pandoc {}",
                command_args_str
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap(),
            );
            let status = command.status().expect(COMMAND_ERROR_MSG);

            if !status.success() {
                log::error!("{}", status);
            }
        }

        Ok(())
    }
}

/// Write the parsed contents into the Pandoc MD file and return that path
/// (`./book/pandoc/md/book.md`)
fn write_pandoc_md_file(dest_path: &Path, parsed_content: &str) -> PathBuf {
    if !dest_path.is_dir() {
        create_dir_all(&dest_path).expect("Error creating the destination directory");
    }
    let md_path = dest_path.join("book.md");

    let mut md_out = File::create(&md_path).expect("Error writing the parsed MD file");
    md_out
        .write_all(parsed_content.as_bytes())
        .expect("Error writing the parsed MD File");

    md_path
}

/// Write and clean the metadata header following these rules:
/// - If the metadata is `null` or empty, nothing is modified.
/// - If the metadata has some values, a new metadata block is added at the
/// start of the document.
fn process_metadata_block(
    mut parsed_contents: String,
    metadata_block: &Option<EpubMetadata>,
) -> String {
    match metadata_block {
        Some(metadata_block) => {
            let metadata_block_str = serde_yaml::to_string(&metadata_block).expect(
                "Error reading the EPUB metadata fields, check the \
            value of the `epub_metadata_fields`.",
            );
            if !metadata_block_str.is_empty() && metadata_block_str.trim() != "{}" {
                parsed_contents = format!("---\n{}---\n\n{}", metadata_block_str, parsed_contents);
            }
        }
        None => {}
    }
    parsed_contents
}
