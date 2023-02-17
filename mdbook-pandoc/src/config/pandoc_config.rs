use super::TomlLoad;
use clap::ValueEnum;
use mdbook_pandoc_derive::pandoc_config_gen;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[pandoc_config_gen("assets/pandoc/configs.toml")]
#[derive(Clone, ValueEnum, Serialize, Deserialize, strum_macros::Display, EnumIter)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum PandocConfig {}
