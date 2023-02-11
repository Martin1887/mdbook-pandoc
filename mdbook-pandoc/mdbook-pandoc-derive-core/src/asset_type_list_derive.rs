//! Derive the implementation of the `AssetTypeList` trait over an enum with
//! a variant for each asset type with the same name that the asset enum but
//! without the `Pandoc` prefix to list the assets information in different
//! formats.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, Ident, ItemEnum};

pub fn asset_type_list_derive_core(input: TokenStream) -> TokenStream {
    let old_enum = match parse2::<ItemEnum>(input) {
        Ok(ast) => ast,
        Err(error) => return error.to_compile_error(),
    };
    impl_asset_type_list_derive(old_enum)
}

fn impl_asset_type_list_derive(ast: ItemEnum) -> TokenStream {
    let enum_name = &ast.ident;
    let enum_variants = ast.variants;

    let mut plain_quote = quote! {};
    let mut json_quote = quote! {};
    let mut yaml_quote = quote! {};

    for var in enum_variants {
        let var_ident = var.ident;
        let asset_enum_ident: Ident = parse_str(&format!("Pandoc{}", var_ident)).unwrap();
        plain_quote.extend(quote! {
            #enum_name::#var_ident => {
                for asset in #asset_enum_ident::iter() {
                    match asset {
                        #asset_enum_ident::Default | #asset_enum_ident::Custom(_) => {},
                        _ => output.push_str(&asset.to_plain())
                    }
                }
            }
        });
        json_quote.extend(quote! {
            #enum_name::#var_ident => {
                for asset in #asset_enum_ident::iter() {
                    match asset {
                        #asset_enum_ident::Default | #asset_enum_ident::Custom(_) => {},
                        _ => output.push_str(&asset.to_json())
                    }
                }
            }
        });
        yaml_quote.extend(quote! {
            #enum_name::#var_ident => {
                for asset in #asset_enum_ident::iter() {
                    match asset {
                        #asset_enum_ident::Default | #asset_enum_ident::Custom(_) => {},
                        _ => output.push_str(&asset.to_yaml())
                    }
                }
            }
        });
    }

    quote! {
        impl AssetTypeList for #enum_name {
            fn to_plain(&self) -> String {
                let mut output = String::new();
                match self {
                    #plain_quote
                }

                output
            }

            fn to_json(&self) -> String {
                let mut output = String::new();
                match self {
                    #json_quote
                }

                output
            }

            fn to_yaml(&self) -> String {
                let mut output = String::new();
                match self {
                    #yaml_quote
                }

                output
            }
        }
    }
}
