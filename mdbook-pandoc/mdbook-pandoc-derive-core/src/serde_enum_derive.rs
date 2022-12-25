//! Derive the implementation of the `Display` trait for enums with a variant
//! having a value called `Custom` printing that value and the serialization of
//! the variant in the rest of variants.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemEnum};

pub fn serde_enum_display_derive_core(input: TokenStream) -> TokenStream {
    let old_enum = match parse2::<ItemEnum>(input) {
        Ok(ast) => ast,
        Err(error) => return error.to_compile_error(),
    };
    impl_serde_enum_display(old_enum)
}

fn impl_serde_enum_display(ast: ItemEnum) -> TokenStream {
    let enum_name = &ast.ident;

    quote! {
        impl Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Custom(v) => write!(f, "{}", v),
                    _ => {
                        write!(f, "{}", serde_yaml::to_string(self).unwrap().trim())
                    }
                }
            }
        }
    }
}
