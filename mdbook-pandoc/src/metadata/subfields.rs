//! Structs defining the subfields for nested metadata fields.

use crate::metadata::keys::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataIdentifierScheme {
    scheme: MetadataSchemeKey,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataTitleType {
    #[serde(rename = "type")]
    type_value: MetadataTitleKey,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataCreatorRole {
    role: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MetadataSubjectType {
    authority: String,
    term: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum MetadataOrientationLock {
    PortraitOnly,
    LandscapeOnly,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum MetadataIbooksScrollAxis {
    #[default]
    Default,
    Vertical,
    Horizontal,
}
