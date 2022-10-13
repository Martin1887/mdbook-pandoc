//! Generate YAML metadata block.

pub(crate) mod fields;
pub(crate) mod keys;
pub(crate) mod subfields;
#[cfg(test)]
mod tests;

use fields::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) identifier: Option<MetadataIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<MetadataTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) creator: Option<MetadataCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) contributor: Option<MetadataCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) subject: Option<MetadataSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub(crate) type_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) relation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) coverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rights: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) belongs_to_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) group_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) css: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_progression_direction: Option<MetadataProgressionDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ibooks: Option<MetadataIbooks>,
}
