//! Metadata configuration objects.

use crate::metadata::Metadata;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum MetadataConfig {
    Path(String),
    Metadata(Metadata),
}
