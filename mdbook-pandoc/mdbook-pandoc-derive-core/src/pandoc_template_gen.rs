//! Generation of the `PandocTemplate` enum variants reading a TOML file
//! deserialized into a `PandocTemplates` struct.
//!
//! In addition of generating the variants, the macro generates the
//! implementation of the following functions (implementing the
//! `PandocResource` trait):
//!
//! - `license`: return the license of the template.
//! - `description`: return a description of the template.
//! - `contents`: return the template contents in bytes.
//! - `filename`: return the name of the file that must be written in the Pandoc
//! templates folder.

use std::{fmt::Display, fs::read_to_string, path::PathBuf};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize, Serializer};
use syn::{parse2, parse_quote, punctuated::Punctuated, token::Comma, ItemEnum, LitStr, Variant};

/// Struct representing the `licenses.toml` file with the specification of all
/// the built-in templates.
#[derive(Serialize, Deserialize)]
pub struct PandocTemplates {
    pub templates: Vec<PandocTemplateSpec>,
}

/// Struct representing the properties (all required) of each template in the
/// `templates.toml` specification file.
#[derive(Serialize, Deserialize)]
pub struct PandocTemplateSpec {
    #[serde(rename(serialize = "name"))]
    #[serde(serialize_with = "map_contents_file_to_snake_case_name")]
    pub contents_file: String,
    pub description: String,
    pub version: String,
    pub docs: String,
    pub dependencies: Vec<String>,
    pub latex_packages: Vec<String>,
    pub license: TemplateLicense,
}

fn map_contents_file_to_snake_case_name<S>(
    contents_file: &str,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let (extension, snake_case_filename) = filename_to_snake_case_name(contents_file);
    serializer.serialize_str(&format!("{}_{}", extension, snake_case_filename))
}

impl Display for PandocTemplateSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = "--------------------------------\n";
        let (extension, snake_case_filename) = filename_to_snake_case_name(&self.contents_file);
        write!(
            f,
            "{}Name: {}\nDescription: {}\nVersion: {}.\nDocs: {}\n{}\nDependencies:\n{}\nLaTeX packages:\n{}\n{}\n",
            separator,
            format!("{}_{}", extension, snake_case_filename),
            self.description,
            self.version,
            self.docs,
            self.license,
            self.dependencies.join("\n"),
            self.latex_packages.join("\n"),
            separator
        )
    }
}

impl PandocTemplateSpec {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_yaml(&self) -> String {
        format!("---\n{}\n...\n", serde_yaml::to_string(self).unwrap())
    }
}

/// Struct representing the license of a template.
#[derive(Serialize, Deserialize)]
pub struct TemplateLicense {
    pub name: String,
    pub url: String,
    pub repository_url: String,
}
impl Display for TemplateLicense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "License: {} ({}).\nRepository URL: {}",
            self.name, self.url, self.repository_url
        )
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    format!(
        "{}{}",
        chars.next().unwrap().to_uppercase(),
        chars.collect::<String>()
    )
}

/// Return extension and snake_case name of an asset from the contents file.
fn filename_to_snake_case_name(filename: &str) -> (String, String) {
    let mut filename_split: Vec<&str> = filename
        .split("/")
        .last()
        .expect("Error getting the filename of a template")
        .split(".")
        .collect();
    let extension = filename_split.pop().unwrap();

    (
        extension.to_string(),
        filename_split.join("_").replace("-", "_"),
    )
}

/// Return the enum variant name transforming the filename (the last part of
/// the path) to CamelCase and putting the extension before the name to ease
/// the identification and sorting of the format.
///
/// The path is assumed to be separated by `/`.
fn filename_to_enum_variant(filename: &str) -> String {
    let (extension, snake_case_name) = filename_to_snake_case_name(filename);
    let name_split = snake_case_name.split("_");
    let mut name = String::new();
    for split in name_split {
        name.push_str(&capitalize(split));
    }

    format!("{}{}", capitalize(&extension), name)
}

pub fn pandoc_template_gen_core(attr: TokenStream, item: TokenStream) -> TokenStream {
    let old_enum = match parse2::<ItemEnum>(item) {
        Ok(ast) => ast,
        Err(error) => return error.to_compile_error(),
    };
    let templates_toml_path = match parse2::<LitStr>(attr) {
        Ok(ast) => ast.value(),
        Err(error) => return error.to_compile_error(),
    };
    impl_pandoc_template_gen(&templates_toml_path, old_enum)
}

fn impl_pandoc_template_gen(templates_toml_path: &str, mut ast: ItemEnum) -> TokenStream {
    let enum_name = &ast.ident;
    let mut enum_variants: Punctuated<Variant, Comma> = Punctuated::new();
    // Add the default and custom variants
    enum_variants.push(parse_quote! {
        /// Use the Pandoc default (not writing the argument).
        #[serde(rename = "")]
        #[default]
        Default
    });
    enum_variants.push(parse_quote! {
        /// In TOML write a custom value with the syntax `{"custom" = "value"}`.
        Custom(String)
    });
    // The `CARGO_MANIFEST_DIR` is of this crate, so the root project directory
    // is one level up.
    let project_root = PathBuf::from(env!["CARGO_MANIFEST_DIR"]).join("..");
    let templates_toml_absolute_path = project_root.join(templates_toml_path);
    let templates: PandocTemplates = toml::from_str(
        &read_to_string(&templates_toml_absolute_path).expect("Error reading the templates file"),
    )
    .expect("Error deserializing the templates file");

    let mut license_quote = quote! {};
    let mut description_quote = quote! {};
    let mut contents_quote = quote! {};
    let mut filename_quote = quote! {};
    let mut to_plain_quote = quote! {};
    let mut to_json_quote = quote! {};
    let mut to_yaml_quote = quote! {};

    let mut first = true;
    for template in templates.templates {
        let name = Ident::new(
            &filename_to_enum_variant(&template.contents_file),
            Span::call_site(),
        );
        let template_description = template.description.clone();
        enum_variants.push(parse_quote! {
            #[doc = #template_description]
            #name
        });
        let license = template.license.to_string();
        let contents_file = templates_toml_absolute_path
            .parent()
            .unwrap()
            .join(&template.contents_file)
            .into_os_string()
            .into_string()
            .expect("Error converting template path to String");
        let filename = template.contents_file.split("/").last().unwrap();
        license_quote.extend(quote! {
            #enum_name::#name => Some(#license),
        });
        description_quote.extend(quote! {
            #enum_name::#name => Some(#template_description),
        });
        contents_quote.extend(quote! {
            #enum_name::#name => Some(include_bytes!(#contents_file).to_vec()),
        });
        filename_quote.extend(quote! {
            #enum_name::#name => Some(#filename),
        });
        let template_plain = template.to_string();
        let template_json = template.to_json();
        let template_yaml = template.to_yaml();
        to_plain_quote.extend(quote! {
            #enum_name::#name => #template_plain.to_string(),
        });
        let json_separator = if first { "" } else { ",\n" };
        to_json_quote.extend(quote! {
            #enum_name::#name => format!("{}{}", #json_separator, #template_json),
        });
        to_yaml_quote.extend(quote! {
            #enum_name::#name => #template_yaml.to_string(),
        });
        first = false;
    }

    ast.variants = enum_variants;
    quote! {
        #ast

        impl PandocResource for #enum_name {
            /// Return a String with the license information of the template.
            fn license(&self) -> Option<&str> {
                match self {
                    #license_quote
                    // Default and custom templates have not a known license
                    _ => None,
                }
            }

            /// Return the description of the template.
            fn description(&self) -> Option<&str> {
                match self {
                    #description_quote
                    // Default and custom templates have not description
                    _ => None,
                }
            }

            /// Return the contents of the template as a vector of bytes.
            fn contents(&self) -> Option<Vec<u8>> {
                match self {
                    #contents_quote
                    // Default and custom templates have not contents
                    // (custom templates files must already exist in the
                    // Pandoc templates directory)
                    _ => None,
                }
            }

            /// Return the filename that must have the template in the Pandoc
            /// templates directory.
            fn filename(&self) -> Option<&str> {
                match self {
                    #filename_quote
                    // For custom templates the filename is the specified one
                    #enum_name::Custom(s) => Some(s),
                    // Default templates have not a filename
                    _ => None,
                }
            }
        }

        impl Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.filename() {
                    Some(filename) => write!(f, "{}", filename),
                    None => write!(f, "")
                }
            }
        }

        impl PandocResourceSpec for #enum_name {
            fn to_plain(&self) -> String {
                match self {
                    #to_plain_quote
                    _ => String::new(),
                }
            }
            fn to_json(&self) -> String {
                match self {
                    #to_json_quote
                    _ => String::new(),
                }
            }
            fn to_yaml(&self) -> String {
                match self {
                    #to_yaml_quote
                    _ => String::new(),
                }
            }
        }
    }
}
