/// Filters and Lua filters.
pub mod filters;
/// Templates and auxiliary files.
pub mod templates;

pub use filters::*;
pub use templates::*;

/// Trait including all the files that can be loaded in the data directory or
/// the `src` folder and embedded in the crate.
pub trait PandocResource {
    /// Return a String with the license information of the template.
    fn license(&self) -> Option<&str>;
    /// Return the description of the template.
    fn description(&self) -> Option<&str>;
    /// Return the contents of the template as a vector of bytes.
    fn contents(&self) -> Option<Vec<u8>>;
    /// Return the filename that must have the template in the Pandoc
    /// templates directory.
    fn filename(&self) -> Option<&str>;
}

/// Trait to list the assets specifications in different formats.
pub trait PandocResourceSpec {
    fn to_plain(&self) -> String;
    fn to_json(&self) -> String;
    fn to_yaml(&self) -> String;
}

#[macro_export]
macro_rules! write_vec_to_src_folder {
    ($struct: ident, $field: ident) => {
        for ass in actual_or_default!($struct, $field) {
            if ass.contents().is_some() {
                std::fs::write(&ass.filename().unwrap(), ass.contents().unwrap())
                    .expect("Error writing the contents of the asset")
            }
        }
    };
}

#[macro_export]
macro_rules! write_arg_to_src_folder {
    ($struct: ident, $field: ident) => {
        let ass = actual_or_default!($struct, $field);
        if ass.contents().is_some() {
            std::fs::write(&ass.filename().unwrap(), ass.contents().unwrap())
                .expect("Error writing the contents of the asset")
        }
    };
}
