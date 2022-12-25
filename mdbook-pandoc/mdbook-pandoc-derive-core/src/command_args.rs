//! Derive the implementation of the `args` function for structs with arguments
//! that must be written only if they don't have a default value. The `args`
//! function receives an `ActualAndDefaultCfg` of that struct as argument and
//! returns a vector containing all arguments.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemStruct};

use crate::{extract_type_from_path, extract_type_path};

pub fn pandoc_command_args_derive_core(input: TokenStream) -> TokenStream {
    let old_struct = match parse2::<ItemStruct>(input) {
        Ok(ast) => ast,
        Err(error) => return error.to_compile_error(),
    };
    impl_pandoc_format_args(old_struct)
}

fn impl_pandoc_format_args(ast: ItemStruct) -> TokenStream {
    let struct_name = &ast.ident;
    let mut args_fill = quote!(let mut args = Vec::new(););
    let type_error_msg = "Fields of the struct must be an option of a type";
    match ast.fields {
        syn::Fields::Named(named_fields) => {
            for field in named_fields.named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                let field_path = extract_type_path(&field.ty).expect("{type_error_msg}");
                let field_type = extract_type_from_path(field_path, "Option", type_error_msg);
                args_fill.extend(quote![
                    let actual_val = actual_or_default!(actual_and_default_cfg, #field_name);
                    if actual_val != <#field_type>::default() {
                        let val_str = actual_val.to_string();
                        // the pandoc argument is the field name in kebab-case
                        let pandoc_arg = stringify!(#field_name).replace('_', "-");
                        if val_str == "true" || val_str == "" {
                            args.push(format!("--{}", pandoc_arg));
                        } else {
                            args.push(format!("--{}={}", pandoc_arg, val_str));
                        }
                    }
                ]);
            }
        }
        _ => panic!("This derive macro only works in structs with fields"),
    };
    quote! {
        impl #struct_name {
            pub fn args(&self, actual_and_default_cfg: &ActualAndDefaultCfg<&#struct_name>) -> Vec<String> {
                #args_fill

                args
            }
        }
    }
}
