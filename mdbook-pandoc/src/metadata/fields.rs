//! Definition of the metadata fields and their conversions from a string for
//! the fields that can be a string or a map.

use crate::metadata::subfields::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MetadataIdentifier {
    Text(String),
    Scheme(MetadataIdentifierScheme),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MetadataTitle {
    Text(String),
    Types(Vec<MetadataTitleType>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MetadataCreator {
    Text(String),
    Roles(Vec<MetadataCreatorRole>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MetadataSubject {
    Text(String),
    Types(Vec<MetadataSubjectType>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MetadataProgressionDirection {
    Ltr,
    Rtl,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct MetadataIbooks {
    pub(crate) version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) specified_fonts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ipad_orientation_lock: Option<MetadataOrientationLock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) iphone_orientation_lock: Option<MetadataOrientationLock>,
    // The additional tuple struct is needed because the default value is true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) binding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) scroll_axis: Option<MetadataIbooksScrollAxis>,
}
