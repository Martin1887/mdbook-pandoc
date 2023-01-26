use std::fmt::Display;

use mdbook_pandoc_derive::pandoc_template_gen;
use serde::{Deserialize, Serialize};

/// Trait including all the files that can be loaded in the data directory or
/// the `src` folder and embedded in the crate.
pub trait PandocResource {
    /// Return a String with the license information of the template.
    fn license(&self) -> Option<&str>;
    /// Return the description of the template.
    fn description(&self) -> Option<&str>;
    /// Return the contents of the template as a vector of bytes.
    fn contents(&self) -> Option<Vec<u8>>;
    /// Return the filename that must have the template in the Pandoc
    /// templates directory.
    fn filename(&self) -> Option<&str>;
}

#[macro_export]
macro_rules! write_vec_to_src_folder {
    ($struct: ident, $field: ident) => {
        for ass in actual_or_default!($struct, $field) {
            if ass.contents().is_some() {
                std::fs::write(&ass.filename().unwrap(), ass.contents().unwrap())
                    .expect("Error writing the contents of the asset")
            }
        }
    };
}

#[macro_export]
macro_rules! write_arg_to_src_folder {
    ($struct: ident, $field: ident) => {
        let ass = actual_or_default!($struct, $field);
        if ass.contents().is_some() {
            std::fs::write(&ass.filename().unwrap(), ass.contents().unwrap())
                .expect("Error writing the contents of the asset")
        }
    };
}

/// Pandoc templates, written in the `templates` subfolder of the data directory.
#[pandoc_template_gen("assets/templates/templates.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocTemplate {}

/// Pandoc files for the `syntax-definition` field, written in the `src` folder.
#[pandoc_template_gen("assets/templates/syntax_definition.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocSyntaxDefinition {}

/// Pandoc files for the `include-in-header` field, written in the `src` folder.
#[pandoc_template_gen("assets/templates/include_in_header.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocIncludeInHeader {}

/// Pandoc files for the `include-before-body` field, written in the `src` folder.
#[pandoc_template_gen("assets/templates/include_before_body.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocIncludeBeforeBody {}

/// Pandoc files after the body, written in the `src` folder.
#[pandoc_template_gen("assets/templates/include_after_body.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocIncludeAfterBody {}

/// Pandoc CSSs for the `css` field, written in the data directory.
#[pandoc_template_gen("assets/templates/css.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocCss {}

/// Pandoc files for the `embed-font` field, written in the `src` folder.
#[pandoc_template_gen("assets/templates/epub_embed_fonts.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocEpubEmbedFont {}

/// Pandoc CSLs, written in the `src` folder.
#[pandoc_template_gen("assets/templates/csl.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocCsl {}

/// Pandoc reference docs, written in the data directory.
#[pandoc_template_gen("assets/templates/reference_docs.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocReferenceDoc {}

/// Pandoc misc assets files (images, CLSs, STYs, etc.), written in the
/// `src` folder of the book.
#[pandoc_template_gen("assets/templates/template_assets.toml")]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PandocTemplateAsset {}
