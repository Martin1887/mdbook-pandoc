//! Configuration structs and parsing.

/// Commands configuration and behavior.
pub mod commands;
/// EPUB metadata.
pub mod epub_metadata;
/// Documents formats.
pub mod formats;
#[cfg(test)]
mod tests;

use commands::*;
use epub_metadata::EpubMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration struct containing all the available options.
/// All options have a default value.
/// Options not set in formats are inherited from the general settings.
#[derive(Serialize, Deserialize)]
pub struct MdBookPandocConfig {
    pub general: GeneralConfig,
    #[serde(flatten)]
    pub commands: HashMap<String, PandocCommand>,
}

/// General configuration that can be overwritten in each format.
#[derive(Serialize, Deserialize)]
pub struct GeneralConfig {
    /// The labels used as parts names to separate preamble, chapters and annexes.
    #[serde(flatten)]
    pub labels: TitleLabels,
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
