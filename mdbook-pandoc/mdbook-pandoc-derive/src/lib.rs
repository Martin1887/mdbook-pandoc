//! Procedural macros for:
//!
//! - Derive the generation of literal and repeated Pandoc arguments getting the
//! actual or the default value for all the fields of the struct.
//!
//! - Derive `Display` for enums with one variant containing an unnamed
//! value called `Custom` printing that value or the serialization of the
//! variant in the rest of variants.
//!
//! - Attribute-like macro to generate the `PandocResource` enums reading a
//! TOML file with the filename, license and other properties of each built-in
//! Pandoc resource.
//!
//! - Derive `AssetTypeList` to print the information about assets in different
//! formats.
//!
//! - Attribute-like macro to generate the `PandocConfig` enum implementing the
//! `TomlLoad` trait to load the configuration file contents.
//!
//! This is a separate crate to ease testing.

use mdbook_pandoc_derive_core::{
    asset_type_list_derive_core, pandoc_command_args_derive_core, pandoc_config_gen_core,
    pandoc_repeated_args_derive_core, pandoc_resource_gen_core, serde_enum_display_derive_core,
};
use proc_macro::TokenStream;

/// Derive the generation of literal arguments iterating over all the fields of
/// a struct and a struct containing the actual and the default (for all
/// formats) values.
///
/// The struct must have only named fields and all fields must be an option of
/// the actual field. The field names must be the actual long argument string
/// (the one preceded by two dashes) in snake_case.
#[proc_macro_derive(PandocCommandArgs)]
pub fn pandoc_command_args_derive(input: TokenStream) -> TokenStream {
    pandoc_command_args_derive_core(input.into()).into()
}

/// Derive the generation of repeated arguments iterating over all the fields of
/// a struct and a struct containing the actual and the default (for all
/// formats) values.
///
/// The struct must have only named fields and all fields must be an option of
/// a vector of the actual args as type implementing `Display`. The field names
/// must be the actual
/// long argument string (the one preceded by two dashes) in snake_case.
#[proc_macro_derive(PandocRepeatedArgs)]
pub fn pandoc_repeated_args_derive(input: TokenStream) -> TokenStream {
    pandoc_repeated_args_derive_core(input.into()).into()
}

/// Derive the `Display` trait for enums with one variant containing an unnamed
/// value called `Custom` printing that value or the serialization of the
/// variant in the rest of variants.
#[proc_macro_derive(SerdeEnumDisplay)]
pub fn serde_enum_display_derive(input: TokenStream) -> TokenStream {
    serde_enum_display_derive_core(input.into()).into()
}

/// Generation of the `PandocResource` variants reading a TOML file. Check the
/// core crate for the full documentation.
#[proc_macro_attribute]
pub fn pandoc_resource_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    pandoc_resource_gen_core(attr.into(), item.into()).into()
}

/// Derive the `AssetTypeList` trait to print assets information in different
/// formats.
#[proc_macro_derive(AssetTypeList)]
pub fn pandoc_asset_type_list_derive(input: TokenStream) -> TokenStream {
    asset_type_list_derive_core(input.into()).into()
}

/// Generation of the `PandocConfig` variants reading a TOML file. Check the
/// core crate for the full documentation.
#[proc_macro_attribute]
pub fn pandoc_config_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    pandoc_config_gen_core(attr.into(), item.into()).into()
}
