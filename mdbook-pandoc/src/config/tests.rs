//! Tests for the configuration parameters

use std::path::PathBuf;

use crate::{
    actual_or_default,
    config::{formats::PandocFormat, *},
};
use toml;

#[test]
fn test_mixed_title_labels() {
    let cfg = r#"
    [general]
    chapters_label = "My chapters"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    assert_eq!(parsed.general.labels.preamble, String::from("Preamble"));
    assert_eq!(parsed.general.labels.chapters, String::from("My chapters"));
    assert_eq!(parsed.general.labels.annexes, String::from("Annexes"));
}

#[test]
fn test_format_extensions() {
    let added_cfg = r#"
    [general]

    [pdf]
    format = "pdf"
    added-extensions = ["one", "plus"]
    "#;
    let added: MdBookPandocConfig = toml::from_str(added_cfg).unwrap();
    let added_command = added.commands.get("pdf").unwrap();
    assert_eq!(
        added_command.args(&PathBuf::new(), "pdf", &added.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=pdf",
            "--output=book.pdf",
            "--from=markdown+one+plus+tex_math_double_backslash",
            "--number-sections",
            "--toc"
        ]
    );
    let added_and_removed_cfg = format!(
        "{}\n{}",
        added_cfg,
        r#"
        removed-extensions = ["minus"]
        "#
    );
    let added_and_removed: MdBookPandocConfig = toml::from_str(&added_and_removed_cfg).unwrap();
    let added_and_removed_command = added_and_removed.commands.get("pdf").unwrap();
    assert_eq!(
        added_and_removed_command.args(&PathBuf::new(), "pdf", &added.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=pdf",
            "--output=book.pdf",
            "--from=markdown+one+plus+tex_math_double_backslash-minus",
            "--number-sections",
            "--toc"
        ]
    );
    let no_double_backslash_cfg = r#"
    [general]

    [pdf]
    format = "pdf"
    removed-extensions = ["tex_math_double_backslash"]
    "#;
    let no_double_backslash: MdBookPandocConfig = toml::from_str(no_double_backslash_cfg).unwrap();
    let no_double_backslash_command = no_double_backslash.commands.get("pdf").unwrap();
    assert_eq!(
        no_double_backslash_command.args(&PathBuf::new(), "pdf", &no_double_backslash.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=pdf",
            "--output=book.pdf",
            "--from=markdown-tex_math_double_backslash",
            "--number-sections",
            "--toc"
        ]
    );
    let from_typst_cfg = r#"
    [general]
    from_format = "typst"

    [pdf]
    format = "pdf"
    "#;
    let from_typst: MdBookPandocConfig = toml::from_str(from_typst_cfg).unwrap();
    let from_typst_command = from_typst.commands.get("pdf").unwrap();
    assert_eq!(
        from_typst_command.args(&PathBuf::new(), "pdf", &from_typst.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=pdf",
            "--output=book.pdf",
            "--from=typst",
            "--number-sections",
            "--toc"
        ]
    );
}

#[test]
fn test_custom_args() {
    let cfg = r#"
    [general]
    filename = "epub"
    from_format = "markua"
    
    [epub]
    format = {"custom" = "epub4"}
    number-sections = false
    toc = true
    extra-args = ["--extra=4", "--extra2"]
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("epub").unwrap();
    assert_eq!(command.format, PandocFormat::Custom("epub4".to_string()));
    assert_eq!(
        parsed.general.formats_default.filename,
        Some(Filename::Custom("epub".to_string()))
    );
    let combined_cfg = ActualAndDefaultCfg {
        actual: Box::new(&command.generic_args),
        default: Box::new(&parsed.general.formats_default),
    };
    assert_eq!(
        actual_or_default!(combined_cfg, filename),
        Filename::Custom(String::from("epub"))
    );
    assert_eq!(
        command.args(&PathBuf::new(), "epub", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=epub4",
            "--output=epub.epub",
            "--extra=4",
            "--extra2",
            "--from=markua",
            "--toc"
        ]
    );
}

#[test]
fn test_repeated_args() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    metadata = ["title:Great"]
    variable = ["boolean_var", "my_var:my_value"]
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--metadata=title:Great",
            "--variable=boolean_var",
            "--variable=my_var:my_value"
        ]
    );
}

#[test]
fn test_template() {
    let cfg = r#"
    [general]
    
    [pdf]
    format = "pdf"
    template = "latex_eisvogel"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("pdf").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "pdf", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=pdf",
            "--output=book.pdf",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--template=eisvogel.latex"
        ]
    );
}

#[test]
fn test_dpi() {
    let cfg = r#"
    [general]
    dpi = 300
    
    [docx]
    format = "docx"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--dpi=300"
        ]
    );
}

#[test]
fn test_highlight_style() {
    let cfg = r#"
    [general]
    
    [pdf]
    format = "pdf"
    highlight-style = "tango"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("pdf").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "pdf", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=pdf",
            "--output=book.pdf",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--highlight-style=tango"
        ]
    );
    let cfg = r#"
    [general]
    
    [pdf]
    format = "pdf"
    highlight-style = {"custom" = "vulcan"}
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("pdf").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "pdf", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=pdf",
            "--output=book.pdf",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--highlight-style=vulcan"
        ]
    );
}

#[test]
fn test_toc_depth_default_display() {
    assert_eq!(
        "0",
        TocDepth::default().to_string(),
        "Error printing `TocDepth`"
    );
}

#[test]
fn test_toc_depth() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    toc-depth = 4
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--toc-depth=4"
        ]
    );
}

#[test]
fn test_reference_location() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    reference-location = "block"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--reference-location=block"
        ]
    );
}

#[test]
fn test_top_level_division() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    top-level-division = "section"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--top-level-division=section"
        ]
    );
}

#[test]
fn test_slide_level() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    slide-level = 2
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--slide-level=2"
        ]
    );
}

#[test]
fn test_email_obfuscation() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    email-obfuscation = "javascript"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--email-obfuscation=javascript"
        ]
    );
}

#[test]
fn test_epub_chapter_level() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    epub-chapter-level = 4
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--epub-chapter-level=4"
        ]
    );
}

#[test]
fn test_epub_subdirectory() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    epub-subdirectory = {"custom" = "other"}
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--epub-subdirectory=other"
        ]
    );
}

#[test]
fn test_ipynb_output() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    ipynb-output = "all"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--ipynb-output=all"
        ]
    );
}

#[test]
fn test_pdf_engine() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    pdf-engine = "tectonic"
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--pdf-engine=tectonic"
        ]
    );
}

#[test]
fn test_tex_url() {
    let cfg = r#"
    [general]
    
    [docx]
    format = "docx"
    katex = ""
    "#;
    let parsed: MdBookPandocConfig = toml::from_str(cfg).unwrap();
    let command = parsed.commands.get("docx").unwrap();
    assert_eq!(
        command.args(&PathBuf::new(), "docx", &parsed.general),
        vec![
            "--embed-resources",
            "--standalone",
            "--eol=lf",
            "--to=docx",
            "--output=book.docx",
            "--from=markdown+tex_math_double_backslash",
            "--number-sections",
            "--toc",
            "--katex"
        ]
    );
}
