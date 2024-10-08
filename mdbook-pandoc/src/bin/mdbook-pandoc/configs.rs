use anyhow::Result;
use std::{
    fs::{read, OpenOptions},
    io::Write,
};
use strum::IntoEnumIterator;
use toml_edit::DocumentMut;

use clap::Subcommand;
use mdbook_pandoc::config::{pandoc_config::PandocConfig, TomlLoad};

#[derive(Subcommand)]
pub(crate) enum ConfigsSubcommand {
    /// Add the contents of a built-in configuration at the end of the file.
    /// The full `output.pandoc` table will
    /// be replaced if the option `clear` is set. An empty
    /// `output.pandoc.general` table is added if it does not exist.
    Load {
        config_to_load: PandocConfig,
        destination_file: Option<String>,
        /// Clear the contents of the `output.pandoc` table.
        #[arg(long)]
        clear: bool,
    },
    /// Add the contents of the provided configuration file at the end. The full
    /// `output.pandoc` table will
    /// be replaced if the option `clear` is set. An empty
    /// `output.pandoc.general` table is added if it does not exist.
    LoadFile {
        config_path: String,
        destination_file: Option<String>,
        /// Clear the contents of the `output.pandoc` table.
        #[arg(long)]
        clear: bool,
    },
    /// List the available configuration files.
    List,
    /// Print the contents of a built-in configuration file.
    Print { config: PandocConfig },
}

/// Read the TOML configuration file of the book and write the contents at the
/// end replacing the whole `output.pandoc` table if `clear` and adding an
/// empty `output.pandoc.general` table if it does not exist.
pub(crate) fn write_in_book_config(
    contents: &[u8],
    destination_file: Option<String>,
    clear: bool,
) -> Result<()> {
    let actual_dest_path = destination_file.unwrap_or("book.toml".to_string());

    let mut general_table_exists = false;
    if let Ok(dest_contents) = read(&actual_dest_path) {
        let mut output_pandoc_exists = false;
        let mut doc = String::from_utf8_lossy(&dest_contents).parse::<DocumentMut>()?;
        if doc.contains_key("output")
            && doc["output"].is_table_like()
            && doc["output"]
                .as_table_like()
                .unwrap()
                .contains_key("pandoc")
        {
            output_pandoc_exists = true;
            if doc["output"]["pandoc"].is_table_like()
                && doc["output"]["pandoc"]
                    .as_table_like()
                    .unwrap()
                    .contains_key("general")
            {
                general_table_exists = true;
            }
        }
        if clear && output_pandoc_exists {
            doc["output"]["pandoc"] = toml_edit::Item::None;
            std::fs::write(&actual_dest_path, doc.to_string().as_bytes())?;
        }
    }

    let mut append_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&actual_dest_path)?;
    // Write an empty `[output.pandoc.general]` table only if the contents
    // and the destination file do not already have it
    let doc = String::from_utf8_lossy(contents).parse::<DocumentMut>()?;
    let contents_contains_general = doc.contains_key("output")
        && doc["output"].is_table_like()
        && doc["output"]
            .as_table_like()
            .unwrap()
            .contains_key("pandoc")
        && doc["output"]["pandoc"].is_table_like()
        && doc["output"]["pandoc"]
            .as_table_like()
            .unwrap()
            .contains_key("general");
    if !contents_contains_general && (!general_table_exists || clear) {
        append_file.write_all("\n[output.pandoc.general]".as_bytes())?;
    }
    append_file.write_all("\n\n".as_bytes())?;
    append_file.write_all(contents)?;
    append_file.write_all("\n".as_bytes())?;

    Ok(())
}

/// List the available configurations.
pub(crate) fn list_configs() -> String {
    PandocConfig::iter()
        .map(|cfg| format!("{}: {}", cfg, cfg.description()))
        .collect::<Vec<String>>()
        .join("\n")
}
