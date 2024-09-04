use std::fmt::Display;

use mdbook_pandoc_derive::SerdeEnumDisplay;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, SerdeEnumDisplay)]
#[serde(rename_all = "snake_case")]
/// Pandoc formats. Note that many formats can be used only as output.
/// mdbook-pandoc assumes Markdown sources and some similar formats and flavours
/// can work as input format but it is not guaranteed.
pub enum PandocFormat {
    Asciidoc,
    Beamer,
    Bibtex,
    Biblatex,
    Chunkedhtml,
    Commonmark,
    CommonMarkX,
    Context,
    Csljson,
    /// Docbook 4.
    Docbook,
    Docbook5,
    Docx,
    Dokuwiki,
    /// EPUB 3.
    Epub,
    Epub2,
    Fb2,
    Gfm,
    MarkdownGithub,
    Haddock,
    /// HTML 5.
    Html,
    Html4,
    Icml,
    Ipynb,
    JatsArchiving,
    JatsArticleauthoring,
    JatsPublishing,
    Jira,
    Json,
    Latex,
    Man,
    Markdown,
    MarkdownMmd,
    MarkdownPhpextra,
    MarkdownStrict,
    Markua,
    Mediawiki,
    Ms,
    Muse,
    Native,
    Odt,
    Opml,
    Opendocument,
    Org,
    Pdf,
    Plain,
    Pptx,
    Rst,
    Rtf,
    Texinfo,
    Textile,
    Typst,
    Dzslides,
    Revealjs,
    Slideous,
    Slidy,
    S5,
    Tei,
    Xwiki,
    Zimwiki,
    /// In TOML write a custom value with the syntax `{"custom" = "value"}`.
    Custom(String),
}
