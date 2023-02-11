//! Core implementation of the arguments derive.

mod asset_type_list_derive;
mod command_args;
mod pandoc_template_gen;
mod repeated_args;
mod serde_enum_derive;

#[cfg(test)]
mod tests;

pub use asset_type_list_derive::*;
pub use command_args::*;
pub use pandoc_template_gen::*;
pub use repeated_args::*;
pub use serde_enum_derive::*;
use syn::{Path, Type};

/// Return the `Path` from a `Type`.
pub(crate) fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Path(ref type_path) if type_path.qself.is_none() => Some(&type_path.path),
        _ => None,
    }
}

/// Return the type inside the path and panic if the container is not the specified.
pub(crate) fn extract_type_from_path<'a>(
    path: &'a Path,
    container_type: &'a str,
    type_error_msg: &'a str,
) -> &'a Type {
    let ps = &path.segments.first().expect("{type_error_msg}");
    if !ps.ident.to_string().contains(container_type) {
        panic!("{type_error_msg}");
    } else {
        match &ps.arguments {
            syn::PathArguments::AngleBracketed(ga) => {
                match ga.args.first().expect("{type_error_msg}") {
                    syn::GenericArgument::Type(ty) => ty,
                    _ => panic!("{type_error_msg}"),
                }
            }
            _ => panic!("{type_error_msg}"),
        }
    }
}
