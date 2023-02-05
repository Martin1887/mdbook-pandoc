use super::*;
use std::fmt::Display;

use mdbook_pandoc_derive::pandoc_template_gen;
use serde::{Deserialize, Serialize};

/// Pandoc templates, written in the `templates` subfolder of the data directory.
#[pandoc_template_gen("assets/pandoc/templates.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocTemplate {}

/// Pandoc files for the `syntax-definition` field, written in the `src` folder.
#[pandoc_template_gen("assets/pandoc/syntax_definition.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocSyntaxDefinition {}

/// Pandoc files for the `include-in-header` field, written in the `src` folder.
#[pandoc_template_gen("assets/pandoc/include_in_header.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocIncludeInHeader {}

/// Pandoc files for the `include-before-body` field, written in the `src` folder.
#[pandoc_template_gen("assets/pandoc/include_before_body.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocIncludeBeforeBody {}

/// Pandoc files after the body, written in the `src` folder.
#[pandoc_template_gen("assets/pandoc/include_after_body.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocIncludeAfterBody {}

/// Pandoc CSSs for the `css` field, written in the data directory.
#[pandoc_template_gen("assets/pandoc/css.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocCss {}

/// Pandoc files for the `embed-font` field, written in the `src` folder.
#[pandoc_template_gen("assets/pandoc/epub_embed_fonts.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocEpubEmbedFont {}

/// Pandoc CSLs, written in the `src` folder.
#[pandoc_template_gen("assets/pandoc/csl.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocCsl {}

/// Pandoc reference docs, written in the data directory.
#[pandoc_template_gen("assets/pandoc/reference_docs.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocReferenceDoc {}

/// Pandoc misc assets files (images, CLSs, STYs, etc.), written in the
/// `src` folder of the book.
#[pandoc_template_gen("assets/pandoc/template_assets.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocTemplateAsset {}
