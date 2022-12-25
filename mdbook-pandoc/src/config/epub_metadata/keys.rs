//! Definition of auxiliary enums for the possible values of field attributes.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(non_camel_case_types)]
pub(crate) enum MetadataSchemeKey {
    ISBN_10,
    GTIN_13,
    UPC,
    ISMN_10,
    DOI,
    LCCN,
    GTIN_14,
    ISBN_13,
    #[serde(rename = "Legal deposit number")]
    LegalDepositNumber,
    URN,
    OCLC,
    ISMN_13,
    ISBN_A,
    JP,
    OLCC,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum MetadataTitleKey {
    Main,
    Subtitle,
    Short,
    Collection,
    Edition,
    Extended,
}
