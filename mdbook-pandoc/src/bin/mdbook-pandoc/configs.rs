use anyhow::{bail, Result};
use std::fs::{read, write};
use strum::IntoEnumIterator;

use clap::Subcommand;
use mdbook_pandoc::config::{pandoc_config::PandocConfig, TomlLoad};

#[derive(Subcommand)]
pub(crate) enum ConfigsSubcommand {
    /// Load a built-in configuration file. The full `output.pandoc` table will
    /// be replaced. If instead you want to merge configuration files replacing
    /// all existing entries, use the `--merge` option.
    Load {
        config_to_load: PandocConfig,
        destination_file: Option<String>,
        /// Merge the contents into the existing configuration file instead of
        /// overwriting the `output.pandoc` table.
        #[arg(long)]
        merge: bool,
    },
    /// Load the contents of the provided configuration file. The full
    /// `output.pandoc` table will
    /// be replaced. If instead you want to merge configuration files replacing
    /// all existing entries, use the `--merge` option.
    LoadFile {
        config_path: String,
        destination_file: Option<String>,
        /// Merge the contents into the existing configuration file instead of
        /// overwriting the `output.pandoc` table.
        #[arg(long)]
        merge: bool,
    },
    /// List the available configuration files.
    List,
    /// Print the contents of a built-in configuration file.
    Print { config: PandocConfig },
}

/// Read the TOML configuration file of the book and write the contents as TOML
/// replacing the whole `output.pandoc` table if `merge` = `false` and replacing
/// only the existing entries and keeping the others otherwise.
pub(crate) fn write_in_book_config(
    contents: &[u8],
    destination_file: Option<String>,
    merge: bool,
) -> Result<()> {
    let actual_dest_path = destination_file.unwrap_or("book.toml".to_string());

    let mut init_config: toml::Value;
    match read(&actual_dest_path) {
        Ok(dest_contents) => {
            init_config = toml::from_str(&String::from_utf8_lossy(&dest_contents))?;
            if !merge {
                if let Some(output) = init_config.get_mut("output") {
                    output["pandoc"] = toml::Value::Table(toml::map::Map::new());
                }
            }
        }
        _ => init_config = toml::from_str("")?,
    }
    let to_merge = toml::from_str(&String::from_utf8_lossy(contents))?;

    let config;
    match serde_toml_merge::merge(init_config, to_merge) {
        Ok(value) => config = value,
        _ => bail!("Error merging configuration files"),
    }

    write(actual_dest_path, toml::to_string_pretty(&config)?)?;

    Ok(())
}

/// List the available configurations.
pub(crate) fn list_configs() -> String {
    PandocConfig::iter()
        .map(|cfg| format!("{}: {}", cfg, cfg.description()))
        .collect::<Vec<String>>()
        .join("\n")
}
