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
    version: String,
    #[serde(default)]
    specified_fonts: bool,
    ipad_orientation_lock: Option<MetadataOrientationLock>,
    iphone_orientation_lock: Option<MetadataOrientationLock>,
    #[serde(default)]
    // The additional tuple struct is needed because the default value is true
    pub(crate) binding: MetadataIbooksBinding,
    #[serde(default)]
    scroll_axis: MetadataIbooksScrollAxis,
}
