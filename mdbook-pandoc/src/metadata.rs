//! Generate YAML metadata block.

mod fields;
mod keys;
mod subfields;
#[cfg(test)]
mod tests;

use fields::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<MetadataIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<MetadataTitle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creator: Option<MetadataCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contributor: Option<MetadataCreator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<MetadataSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rights: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    belongs_to_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_progression_direction: Option<MetadataProgressionDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ibooks: Option<MetadataIbooks>,
}
