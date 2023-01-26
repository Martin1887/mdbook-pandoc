use crate::{actual_or_default, config::templates::*};
use mdbook_pandoc_derive::{PandocCommandArgs, PandocRepeatedArgs};
use serde::{Deserialize, Serialize};

use super::{defaultable::ActualAndDefaultCfg, pandoc_args::*};

/// Struct defining generic commands arguments. This struct is used in the
/// general configuration and in each format, defaulting to the generic when
/// not set and the default value if also not set in the general configuration.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PandocCommandSharedParameters {
    /// Name of the output file, `book` by default. To add sub-extensions
    /// set this parameter. For example, to generate a PDF output with the
    /// name `my_book.latex.pdf` in the entry `output.pandoc.pdf` set the field
    /// `filename` to `my_book.latex` and the `format` to `pdf`.
    ///
    /// Another manner of having sub-extensions is putting the key of the table
    /// inside quotes, e.g. "pandoc.latex.pdf". This manner allows generating
    /// several files with the same final extension.
    pub filename: Option<Filename>,
    /// The added extensions to the source format without the `+` character.
    pub added_extensions: Option<Vec<String>>,
    /// The removed extensions to the source format without the `-` character.
    pub removed_extensions: Option<Vec<String>>,
    /// Extra arguments defined by pandoc but not in mdbook-pandoc to be
    /// literally added to the command.
    /// E.g., `["--log=pandoc_log.json", "--ascii", "--embed-resources"]`.
    pub extra_args: Option<Vec<String>>,
    /// If the sections are numbered, `true` by default.
    pub number_sections: Option<DefaultTrueBool>,
    /// If a table of contents is included, `true` by default.
    pub toc: Option<DefaultTrueBool>,
    /// Additional assets files, written in the `src` folder of the book.
    pub assets: Option<Vec<PandocTemplateAsset>>,
    /// Pandoc command arguments that can appear several times.
    #[serde(flatten)]
    pub repeated_args: PandocRepeatedArguments,
    /// Pandoc command arguments with the same defaults that Pandoc not written
    /// when they have the default value and directly written otherwise.
    #[serde(flatten)]
    pub args: PandocCommandArguments,
}

/// Struct defining the arguments that can appear several times, so each value
/// of the vector is written. All fields must be an `Option<Vec<String>>`.
#[derive(PandocRepeatedArgs, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PandocRepeatedArguments {
    /// Specify metadata YAML or JSON files with the metadata values.
    pub metadata_file: Option<Vec<String>>,
    /// Specify metadata values that overwrite `epub_metadata` values in format
    /// `key` or `key:val`.
    pub metadata: Option<Vec<String>>,
    /// Template variables values in format `key` or `key:val`.
    pub variable: Option<Vec<String>>,
    /// Syntax definitions XML files.
    pub syntax_definition: Option<Vec<PandocSyntaxDefinition>>,
    /// Files whose contents are included in the header of HTML documents.
    pub include_in_header: Option<Vec<PandocIncludeInHeader>>,
    /// Files whose contents are included just after the `body` tag in HTML and
    /// LaTeX.
    pub include_before_body: Option<Vec<PandocIncludeBeforeBody>>,
    /// Files whose contents are included just before the closing `body` tag
    /// in HTML and LaTeX.
    pub include_after_body: Option<Vec<PandocIncludeAfterBody>>,
    /// List of paths to search resources.
    pub resource_path: Option<Vec<String>>,
    /// List of CSS files.
    pub css: Option<Vec<PandocCss>>,
    /// List of fonts to be embedded in the EPUB.
    pub epub_embed_font: Option<Vec<PandocEpubEmbedFont>>,
    /// Filter to transform the input after the Pandoc parsing.
    pub filter: Option<Vec<String>>,
    /// Filter to transform the input after the Pandoc parsing with a Lua filter.
    pub lua_filter: Option<Vec<String>>,
}

/// Struct defining the Pandoc arguments that are literally written and have
/// the same defaults of Pandoc, so default value means no writing them.
/// All fields must be an `Option` of a type with default value.
#[derive(PandocCommandArgs, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PandocCommandArguments {
    /// Pandoc data directory, by default empty (using Pandoc defaults).
    pub data_dir: Option<String>,
    /// YAML file path with default settings for not specified arguments.
    pub defaults: Option<String>,
    /// Verbose debugging output.
    pub verbose: Option<bool>,
    /// Suppress warning messages.
    pub quiet: Option<bool>,
    /// Exit with error status if there are any warnings.
    pub fail_if_warnings: Option<bool>,
    /// Write log messages in machine-readable JSON format to this path.
    pub log: Option<String>,
    /// Default extension for images without extensions. Useful to use different
    /// images in each output format.
    pub default_image_extension: Option<String>,
    /// Preserve tabs instead converting them to spaces.
    pub preserve_tabs: Option<bool>,
    /// Directory where extract images and other media.
    pub extract_media: Option<String>,
    /// Abbreviations file path.
    pub abbreviations: Option<String>,
    /// Run Pandoc in a sandbox.
    pub sandbox: Option<bool>,
    /// Default images DPI, 96 by default.
    pub dpi: Option<Dpi>,
    /// Disable the highlight of source code blocks.
    pub no_highlight: Option<bool>,
    /// Highlight style for source code, `pygments` by default.
    pub highlight_style: Option<HighlightStyle>,
    /// Custom template for the generated document.
    pub template: Option<PandocTemplate>,
    /// The TOC depth, a value between 1 and 6, `3` by default.
    pub toc_depth: Option<TocDepth>,
    /// Request headers in format `NAME:VAL`.
    pub request_header: Option<String>,
    /// No check certificate to access unsecure HTTP resources.
    pub no_check_certificate: Option<bool>,
    /// Embed resources in `data:` URIs, only valid for HTML output formats.
    pub embed_resources: Option<bool>,
    /// `<q>` tags for quotes in HTML, it only has effect if `smart` extension
    /// is enabled.
    pub html_q_tags: Option<bool>,
    /// Use only ASCII characters.
    pub ascii: Option<bool>,
    /// Use reference links instead of inline links in MD and reStructuredText.
    pub reference_links: Option<bool>,
    /// Reference location for footnotes and references.
    pub reference_location: Option<ReferenceLocation>,
    /// The top-level-division, by default `part` when the book has parts and
    /// `chapter` otherwise.
    pub top_level_division: Option<TopLevelDivision>,
    /// Use `listings` package for LaTeX code blocks.
    pub listings: Option<bool>,
    /// Show list items in slides incrementally.
    pub incremental: Option<bool>,
    /// Slide level for slides, number from 0 to 6 or empty for automatic.
    pub slide_level: Option<SlideLevel>,
    /// Wrap sections in `<section>` tags and attach identifiers to it instead
    /// of the header.
    pub section_divs: Option<bool>,
    /// E-mail obfuscation, default value is `none`.
    pub email_obfuscation: Option<EmailObfuscation>,
    /// Prefix for all identifiers and internal links.
    pub id_prefix: Option<String>,
    /// Prefix for HTML title headers (but not for titles in the body).
    pub title_prefix: Option<String>,
    /// File used as style reference for docx, pptx, potx and odt.
    pub reference_doc: Option<PandocReferenceDoc>,
    /// EPUB cover image, overwriting the specified in the metadata.
    pub epub_cover_image: Option<String>,
    /// EPUB metadata XML file, whose values overwrite the specified ones in the
    /// general configuration, since they are written as YAML metadata block.
    pub epub_metadata: Option<String>,
    /// Number of heading level in which the EPUB is splitted in chapters.
    /// The default value is 1.
    pub epub_chapter_level: Option<EpubChapterLevel>,
    /// EPUB subdirectory in the OCF container, `EPUB` by default, empty to
    /// put EPUB contents in the top level.
    pub epub_subdirectory: Option<EpubSubdirectory>,
    /// How ipynb output cells are treated. The default is `best`.
    pub ipynb_output: Option<IpynbOutput>,
    /// PDF engine for PDF outputs. Empty for Pandoc default.
    pub pdf_engine: Option<PdfEngine>,
    /// Command-line argument for the PDF engine.
    pub pdf_engine_opt: Option<String>,
    /// Process the citations in the files.
    pub citeproc: Option<bool>,
    /// Bibliography file overriding any value set in the metadata.
    pub bibliography: Option<String>,
    /// CSL metadata field as the specified file (equivalent to
    /// `--metadata csl=FILE`), overriding any value set in the metadata.
    pub csl: Option<PandocCsl>,
    /// `citations-abbreviations` metadata field as the specified file
    /// (equivalent to `--metadata citation-abbreviations=FILE).
    pub citation_abbreviations: Option<String>,
    /// Use `natbib` for citations in LaTeX output. Intended only for producing
    /// LaTeX files that can be processed with `bibtex`.
    pub natbib: Option<bool>,
    /// Use `biblatex` for citations in LaTeX output. Intended only for
    /// producing LaTeX files that can be processed with `biblatex` or `biber`.
    pub biblatex: Option<bool>,
    /// Use Mathjax to display embedded TeX math in HTML output.
    pub mathjax: Option<bool>,
    /// Convert TeX math to MathML, the default in `odt` output.
    pub mathml: Option<bool>,
    /// Convert TeX math to `<img>` tags linked to an external script that
    /// convert them. Formulas wil be URL-encoded and concatenated with the
    /// URL provided. The default value is `none` (not adding the argument)
    /// and an empty string will use the Pandoc default URL.
    pub webtex: Option<TexUrl>,
    /// Use KaTeX to display TeX math in HTML output. The URL is the base URL
    /// for the KaTeX library. The default value is `none` (not adding the
    /// argument) and an empty string will be use the Pandoc default URL.
    pub katex: Option<TexUrl>,
    /// Enclose TeX math in `<eq>` tags in HTML output.
    pub gladtex: Option<bool>,
}
