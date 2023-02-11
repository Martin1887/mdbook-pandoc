use super::*;
use std::fmt::Display;

use mdbook_pandoc_derive::pandoc_template_gen;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Pandoc filters, written in the `filters` subfolder of the data directory.
#[pandoc_template_gen("assets/pandoc/filters.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum PandocFilter {}

/// Pandoc Lua filters, written in the `filters` subfolder of the data directory.
#[pandoc_template_gen("assets/pandoc/lua_filters.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum PandocLuaFilter {}
