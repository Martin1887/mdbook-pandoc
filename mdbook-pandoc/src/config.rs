//! Configuration structs and parsing.

/// Commands configuration and behavior.
pub mod commands;
/// EPUB metadata.
pub mod epub_metadata;
/// Documents formats.
pub mod formats;
/// Pandoc built-in (and custom) configurations.
pub mod pandoc_config;
/// Pandoc resources including templates and filters.
pub mod resources;
#[cfg(test)]
mod tests;

use commands::*;
use epub_metadata::EpubMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Load a TOML chunk and provide description to it.
pub trait TomlLoad {
    /// Load the contents of the TOML into a vector of bytes.
    fn load(&self) -> Vec<u8>;
    /// Description of what the configuration does.
    fn description(&self) -> String;
}

/// Configuration struct containing all the available options.
/// All options have a default value.
/// Options not set in formats are inherited from the general settings.
#[derive(Serialize, Deserialize)]
pub struct MdBookPandocConfig {
    /// General configuration of mdbook-pandoc.
    pub general: GeneralConfig,
    /// Commands to be executed and their parameters.
    #[serde(flatten)]
    pub commands: HashMap<String, PandocCommand>,
}

/// General configuration that can be overwritten in each format.
#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Log level, `INFO` by default.
    #[serde(default = "GeneralConfig::default_log_level")]
    pub log_level: log::LevelFilter,
    /// The labels used as parts names to separate preamble, chapters and annexes.
    #[serde(flatten)]
    pub labels: TitleLabels,
    /// Add ` .unnumbered .unlisted` attributes to headings that are not the main
    /// one in each chapter (`true` by default). Disabling this can be useful in
    /// slideshows and other document types or if the full book is written in
    /// only one MD file.
    #[serde(default = "GeneralConfig::default_unlist_not_main_headings")]
    pub unlist_not_main_headings: bool,
    /// The source format, `markdown` by default. Note that all source formats
    /// may not work because the mdbook-pandoc transformations.
    #[serde(default = "GeneralConfig::default_from_format")]
    pub from_format: String,
    /// The EPUB metadata fields written as a YAML metadata block before
    /// executing Pandoc.
    #[serde(default)]
    pub epub_metadata_fields: Option<EpubMetadata>,
    /// Default parameters for all formats.
    #[serde(flatten)]
    pub formats_default: PandocCommandSharedParameters,
}

impl GeneralConfig {
    /// Return the default log level (`INFO`)
    pub fn default_log_level() -> log::LevelFilter {
        log::LevelFilter::Info
    }

    /// Return if unlist not main headings by default (`true`)
    pub fn default_unlist_not_main_headings() -> bool {
        true
    }

    /// Return the default source format (`markdown)
    pub fn default_from_format() -> String {
        String::from("markdown")
    }
}

/// Labels for the titles.
#[derive(Serialize, Deserialize)]
pub struct TitleLabels {
    #[serde(default = "TitleLabels::preamble_default")]
    #[serde(rename = "preamble_label")]
    pub preamble: String,
    #[serde(default = "TitleLabels::chapters_default")]
    #[serde(rename = "chapters_label")]
    pub chapters: String,
    #[serde(default = "TitleLabels::annexes_default")]
    #[serde(rename = "annexes_label")]
    pub annexes: String,
}

impl TitleLabels {
    pub fn preamble_default() -> String {
        String::from("Preamble")
    }
    pub fn chapters_default() -> String {
        String::from("Chapters")
    }
    pub fn annexes_default() -> String {
        String::from("Annexes")
    }
}
