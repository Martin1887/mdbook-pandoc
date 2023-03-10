//! Core implementation of the arguments derive.

mod asset_type_list_derive;
mod command_args;
mod pandoc_config_gen;
mod pandoc_resource_gen;
mod repeated_args;
mod serde_enum_derive;

#[cfg(test)]
mod tests;

pub use asset_type_list_derive::*;
pub use command_args::*;
pub use pandoc_config_gen::*;
pub use pandoc_resource_gen::*;
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

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    format!(
        "{}{}",
        chars.next().unwrap().to_uppercase(),
        chars.collect::<String>()
    )
}

/// Return extension and snake_case name of a resource from the contents file.
fn filename_to_snake_case_name(filename: &str) -> (String, String) {
    let mut filename_split: Vec<&str> = filename
        .split('/')
        .last()
        .expect("Error getting the filename of a resource")
        .split('.')
        .collect();
    let extension = filename_split.pop().unwrap();

    (
        extension.to_string(),
        filename_split.join("_").replace('-', "_"),
    )
}

/// Return the enum variant name transforming the filename (the last part of
/// the path) to CamelCase and putting the extension before the name to ease
/// the identification and sorting of the format.
///
/// The path is assumed to be separated by `/`.
fn filename_to_enum_variant(filename: &str, prepend_extension: bool) -> String {
    let (extension, snake_case_name) = filename_to_snake_case_name(filename);
    let name_split = snake_case_name.split('_');
    let mut name = String::new();
    for split in name_split {
        name.push_str(&capitalize(split));
    }

    if prepend_extension {
        format!("{}{}", capitalize(&extension), name)
    } else {
        name
    }
}
