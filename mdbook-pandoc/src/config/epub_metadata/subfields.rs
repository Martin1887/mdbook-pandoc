//! Structs defining the subfields for nested metadata fields.

use super::keys::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetadataIdentifierScheme {
    scheme: MetadataSchemeKey,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataTitleType {
    #[serde(rename = "type")]
    type_value: MetadataTitleKey,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataCreatorRole {
    role: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataSubjectType {
    authority: String,
    term: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetadataOrientationLock {
    PortraitOnly,
    LandscapeOnly,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum MetadataIbooksScrollAxis {
    #[default]
    Default,
    Vertical,
    Horizontal,
}
