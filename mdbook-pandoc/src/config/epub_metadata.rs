//! Generate YAML metadata block.

pub mod fields;
pub mod keys;
pub mod subfields;
#[cfg(test)]
mod tests;

use fields::*;
use serde::{Deserialize, Serialize};

/// Struct containing all metadata fields. All arguments are optional and not
/// written when their value is `None`.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct EpubMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<MetadataIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<MetadataTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<MetadataCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor: Option<MetadataCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<MetadataSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belongs_to_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_progression_direction: Option<MetadataProgressionDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibooks: Option<MetadataIbooks>,
}
