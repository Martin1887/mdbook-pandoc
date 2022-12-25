//! Definition of the metadata fields and their conversions from a string for
//! the fields that can be a string or a map.

use super::subfields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum MetadataIdentifier {
    Text(String),
    Scheme(MetadataIdentifierScheme),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum MetadataTitle {
    Text(String),
    Types(Vec<MetadataTitleType>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum MetadataCreator {
    Text(String),
    Roles(Vec<MetadataCreatorRole>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum MetadataSubject {
    Text(String),
    Types(Vec<MetadataSubjectType>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetadataProgressionDirection {
    Ltr,
    Rtl,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataIbooks {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specified_fonts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipad_orientation_lock: Option<MetadataOrientationLock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iphone_orientation_lock: Option<MetadataOrientationLock>,
    // The additional tuple struct is needed because the default value is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_axis: Option<MetadataIbooksScrollAxis>,
}
