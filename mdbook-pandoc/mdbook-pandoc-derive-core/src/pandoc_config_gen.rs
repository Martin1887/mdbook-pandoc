//! Generation of the `PandocConfig` variants reading a TOML file
//! deserialized into a `PandocConfigs` struct.
//!
//! The implementation of the `TomlLoad` trait is also derived.

use std::{fs::read_to_string, path::PathBuf};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::{parse2, parse_quote, punctuated::Punctuated, token::Comma, ItemEnum, LitStr, Variant};

use crate::filename_to_enum_variant;

/// Struct representing the files with the specification of all
/// the built-in resources.
#[derive(Serialize, Deserialize)]
pub struct PandocConfigs {
    pub configs: Vec<PandocConfigSpec>,
}

/// Struct representing the properties (all required) of each config in the
/// TOML specification file.
#[derive(Serialize, Deserialize)]
pub struct PandocConfigSpec {
    pub contents_file: String,
    pub description: String,
}

pub fn pandoc_config_gen_core(attr: TokenStream, item: TokenStream) -> TokenStream {
    let old_enum = match parse2::<ItemEnum>(item) {
        Ok(ast) => ast,
        Err(error) => return error.to_compile_error(),
    };
    let configs_toml_path = match parse2::<LitStr>(attr) {
        Ok(ast) => ast.value(),
        Err(error) => return error.to_compile_error(),
    };
    impl_pandoc_config_gen(&configs_toml_path, old_enum)
}

fn impl_pandoc_config_gen(resources_toml_path: &str, mut ast: ItemEnum) -> TokenStream {
    let enum_name = &ast.ident;
    let mut enum_variants: Punctuated<Variant, Comma> = Punctuated::new();
    // The `CARGO_MANIFEST_DIR` is of this crate, so the root project directory
    // is one level up.
    let project_root = PathBuf::from(env!["CARGO_MANIFEST_DIR"]).join("..");
    let resources_toml_absolute_path = project_root.join(resources_toml_path);
    let configs: PandocConfigs = toml::from_str(
        &read_to_string(&resources_toml_absolute_path).expect("Error reading the resources file"),
    )
    .expect("Error deserializing the resources file");

    let mut description_quote = quote! {};
    let mut contents_quote = quote! {};

    for config in configs.configs {
        let name = Ident::new(
            &filename_to_enum_variant(&config.contents_file, false),
            Span::call_site(),
        );
        let description = config.description.clone();
        enum_variants.push(parse_quote! {
            #[doc = #description]
            #name
        });
        let contents_file = resources_toml_absolute_path
            .parent()
            .unwrap()
            .join(&config.contents_file)
            .into_os_string()
            .into_string()
            .expect("Error converting resource path to String");
        description_quote.extend(quote! {
            #enum_name::#name => #description.to_string(),
        });
        contents_quote.extend(quote! {
            #enum_name::#name => include_bytes!(#contents_file).to_vec(),
        });
    }

    ast.variants = enum_variants;
    quote! {
        #ast

        impl TomlLoad for #enum_name {
            /// Return the description of the configuration.
            fn description(&self) -> String {
                match self {
                    #description_quote
                }
            }

            /// Return the contents of the configuration file as a vector of bytes.
            fn load(&self) -> Vec<u8> {
                match self {
                    #contents_quote
                }
            }
        }
    }
}
