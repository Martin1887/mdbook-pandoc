//! mdBook backend that is able to generate many output formats using `pandoc`.
//! Note that `pandoc` must be installed in your system. You also need other
//! tools for some formats like PDF.
//!
//! The following opinionated rules are applied to the book, supposing that
//! headings that are not the main heading are used only to highlight and
//! separate elements in the page but they must not be in the table of contents
//! nor be numbered (if an actual title is desired its contents should be in a
//! different Markdown file):
//! - If the book have prefixes or suffixes a part is created for each of them.
//! - If parts are created for prefixes and/or suffixes and the numbered
//! chapters don't have parts then a part is created for the chapters, the
//! original parts are used for the numbered chapters otherwise.
//! - The heading of the files are downgraded adding the level of the file
//! (e.g. a chapter inside a part has level 1 so its 1-level heading becomes 2-level).
//! - If a heading's level becomes bigger than 6 then the text is simply a
//! different paragraph with bold text.
//! - Setext headings (underlined ones) are changed to ATX (hashes prefix).
//! - Headings that are not the main one are labeled with `{.unnumbered .unlisted}`
//! to remove numbers and avoid that they appear in the table of contents unless
//! this behaviour is disabled setting as `false` the `unlist_not_main_headings`
//! configuration parameter.
//! - Each Markdown file must have a main 1st level heading as the first heading
//! of the file and that heading is used as its heading, the name of the file is
//! ignored.

pub mod config;
pub mod parse;
#[cfg(test)]
mod tests;

#[macro_use]
extern crate lazy_static;

use std::{
    cmp::Ordering,
    collections::HashMap,
    env::set_current_dir,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::Local;
use config::{
    commands::PandocCommand, epub_metadata::EpubMetadata, GeneralConfig, MdBookPandocConfig,
};
use log::warn;
use mdbook::{renderer::RenderContext, Renderer};
use parse::parse_book;

const COMMAND_ERROR_MSG: &str = r#"Error launching a Pandoc command, check that \
Pandoc is installed in your system and available in your path as `pandoc`"#;

/// Initialize the log.
pub fn init_logger(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();
    builder.filter_level(level);
    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] (mdbook_pandoc): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            formatter.default_styled_level(record.level()),
            record.args()
        )
    });

    builder
}

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
        let cfg = ctx
            .config
            .get_deserialized_opt::<MdBookPandocConfig, &str>("output.pandoc")
            .expect("Error reading the configuration file")
            .unwrap();
        let general_cfg = cfg.general;
        let title_labels = &general_cfg.labels;
        let unlist_headings: bool = general_cfg.unlist_not_main_headings;

        let log_result = init_logger(general_cfg.log_level).try_init();
        if log_result.is_err() {
            warn!("Error initializing the log.");
        }

        let parsed = process_metadata_block(
            parse_book(ctx, title_labels, unlist_headings),
            &general_cfg.epub_metadata_fields,
        );
        let parsed_md_path = write_pandoc_md_file(&ctx.destination, &parsed);

        // Execute the commands
        pandoc_commands(
            &ctx.source_dir(),
            &ctx.destination,
            &parsed_md_path,
            &general_cfg,
            &cfg.commands,
        )?;

        Ok(())
    }
}

/// Execute the Pandoc commands in the correct order.
fn pandoc_commands(
    source_dir: &Path,
    destination: &Path,
    parsed_md_path: &Path,
    general_cfg: &GeneralConfig,
    commands: &HashMap<String, PandocCommand>,
) -> mdbook::errors::Result<()> {
    // the path must be the book `src` folder because paths are relative to
    // that directory
    set_current_dir(source_dir)?;

    // sort commands using the `order` field, where a 0 value is like MAX
    // (irrelevant order, can be put at the end)
    let mut sorted_commands = commands.iter().collect::<Vec<(&String, &PandocCommand)>>();
    sorted_commands.sort_unstable_by(|a, b| {
        match (a.1.generic_args.order, b.1.generic_args.order) {
            (0, 0) => Ordering::Equal,
            (0, _) => Ordering::Greater,
            (_, 0) => Ordering::Less,
            (x, y) => x.partial_cmp(&y).unwrap(),
        }
    });

    for (extension, pandoc_command) in sorted_commands {
        execute_pandoc_command(
            destination,
            parsed_md_path,
            extension,
            general_cfg,
            pandoc_command,
        );
    }

    Ok(())
}

/// Execute the specified Pandoc command.
fn execute_pandoc_command(
    destination: &Path,
    parsed_md_path: &Path,
    extension: &str,
    general_cfg: &GeneralConfig,
    pandoc_command: &PandocCommand,
) {
    pandoc_command.write_template_and_assets_files(general_cfg);
    let mut command = pandoc_command.command(destination, parsed_md_path, extension, general_cfg);
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
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap(),
    );
    let status = command.status().expect(COMMAND_ERROR_MSG);

    if !status.success() {
        log::error!("{}", status);
    }
}

/// Write the parsed contents into the Pandoc MD file and return that path
/// (`./book/pandoc/md/book.md`).
fn write_pandoc_md_file(dest_path: &Path, parsed_content: &str) -> PathBuf {
    if !dest_path.is_dir() {
        create_dir_all(dest_path).expect("Error creating the destination directory");
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
