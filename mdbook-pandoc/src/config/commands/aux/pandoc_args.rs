use mdbook_pandoc_derive::SerdeEnumDisplay;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;

/// The filename is a `String` that is `book` by default.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerdeEnumDisplay)]
#[serde(untagged)]
pub enum Filename {
    Custom(String),
}
impl Default for Filename {
    fn default() -> Self {
        Filename::Custom(String::from("book"))
    }
}

/// A wrapper for a boolean with `true` as default value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefaultTrueBool {
    Value(bool),
}
impl Default for DefaultTrueBool {
    fn default() -> Self {
        DefaultTrueBool::Value(true)
    }
}

/// The DPI is a number that by default is 96.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerdeEnumDisplay)]
#[serde(untagged)]
pub enum Dpi {
    Custom(u32),
}
impl Default for Dpi {
    fn default() -> Self {
        Dpi::Custom(96)
    }
}

/// Possible values for the wrap options.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Wrap {
    /// The default value, try to wrap lines to `--columns` value (72 by default).
    #[default]
    Auto,
    /// Not wrap lines at all.
    None,
    /// Attempt to preserve the wrapping from the source document.
    Preserve,
}

/// The number of columns for wrapped text, 72 by default.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerdeEnumDisplay)]
#[serde(untagged)]
pub enum Columns {
    Custom(u32),
}
impl Default for Columns {
    fn default() -> Self {
        Columns::Custom(72)
    }
}

/// Possible values for the highlight style.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, SerdeEnumDisplay)]
#[serde(rename_all = "camelCase")]
pub enum HighlightStyle {
    /// The default value.
    #[default]
    Pygments,
    Kate,
    Monochrome,
    Breezedark,
    Espresso,
    Zenburn,
    Haddock,
    Tango,
    /// In TOML write it with the syntax `{"custom" = "path"}`.
    Custom(String),
}

/// Possible values for the TOC depth, being `0` the default value
/// (argument not written using the Pandoc default).
#[derive(
    Clone, Debug, Default, PartialEq, Serialize_repr, Deserialize_repr, strum_macros::Display,
)]
#[repr(u8)]
pub enum TocDepth {
    /// The default value, not writing it as argument and therefore using the
    /// Pandoc default.
    #[default]
    #[strum(serialize = "0")]
    Default = 0,
    #[strum(serialize = "1")]
    One = 1,
    #[strum(serialize = "2")]
    Two = 2,
    #[strum(serialize = "3")]
    Three = 3,
    #[strum(serialize = "4")]
    Four = 4,
    #[strum(serialize = "5")]
    Five = 5,
    #[strum(serialize = "6")]
    Six = 6,
}

/// Possible values for the `reference-location` field, being `document` the
/// default value.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ReferenceLocation {
    /// The default value.
    #[default]
    Document,
    Block,
    Section,
}

/// Values for the top-level-division.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TopLevelDivision {
    /// `default` defined by pandoc.
    #[default]
    Default,
    Section,
    Chapter,
    Part,
}

/// Values for slide-level, default value is -1 (the argument is not written
/// so the Pandoc default is used).
#[derive(
    Clone, Debug, Default, PartialEq, Serialize_repr, Deserialize_repr, strum_macros::Display,
)]
#[repr(i8)]
pub enum SlideLevel {
    /// The default value, not writing the argument.
    #[default]
    #[strum(serialize = "-1")]
    Auto = -1,
    #[strum(serialize = "0")]
    Zero = 0,
    #[strum(serialize = "1")]
    One = 1,
    #[strum(serialize = "2")]
    Two = 2,
    #[strum(serialize = "3")]
    Three = 3,
    #[strum(serialize = "4")]
    Four = 4,
    #[strum(serialize = "5")]
    Five = 5,
    #[strum(serialize = "6")]
    Six = 6,
}

/// Email obfuscation options.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum EmailObfuscation {
    /// No obfuscation, the default value.
    #[default]
    None,
    /// JavaScript based obfuscation.
    Javascript,
    /// Print letters as decimal or hexadecimal character references.
    References,
}

/// Heading level in which split EPUB in chapters, `1` by default.
#[derive(
    Clone, Debug, Default, PartialEq, Serialize_repr, Deserialize_repr, strum_macros::Display,
)]
#[repr(u8)]
pub enum EpubChapterLevel {
    #[default]
    #[strum(serialize = "1")]
    One = 1,
    #[strum(serialize = "2")]
    Two = 2,
    #[strum(serialize = "3")]
    Three = 3,
    #[strum(serialize = "4")]
    Four = 4,
    #[strum(serialize = "5")]
    Five = 5,
    #[strum(serialize = "6")]
    Six = 6,
}

/// EPUB subdirectory, `EPUB` by default and empty to put all contents at
/// the top level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, SerdeEnumDisplay)]
#[serde(rename_all = "kebab-case")]
pub enum EpubSubdirectory {
    #[default]
    #[serde(rename = "EPUB")]
    Epub,
    #[serde(rename = "")]
    Empty,
    /// In TOML write it with the syntax `{"other" = "value"}`.
    Custom(String),
}

/// How ipynb output cells are treated. The default is `best`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum IpynbOutput {
    /// Pandoc tries to pick the compatible richest data block in each output
    /// cell. The default value.
    #[default]
    Best,
    /// All data formats are preserved.
    All,
    /// Data cells contents are omitted.
    None,
}

/// PDF engine for PDF outputs. Empty for Pandoc default.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, SerdeEnumDisplay)]
#[serde(rename_all = "kebab-case")]
pub enum PdfEngine {
    /// Default engine for the output format (the argument is not written).
    #[default]
    #[serde(rename = "")]
    Default,
    Pdflatex,
    Lualatex,
    Xelatex,
    Latexmk,
    Tectonic,
    Wkhtmltopdf,
    Weasyprint,
    PagedjsCli,
    Prince,
    Context,
    Pdfroff,
    /// Path to the executable of one of the previous options.
    /// In TOML write it with the syntax `{"custom" = "path"}`.
    Custom(String),
}

/// Webtex and Katex options.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum TexUrl {
    /// The argument is not added.
    #[default]
    #[serde(rename = "none")]
    #[strum(serialize = "none")]
    None,
    /// Default URL (empty value in the Pandoc command).
    #[serde(rename = "")]
    #[strum(serialize = "")]
    Empty,
    /// Custom URL, in TOML write it with the syntax `{"custom" = "url"}`.
    Custom(String),
}
