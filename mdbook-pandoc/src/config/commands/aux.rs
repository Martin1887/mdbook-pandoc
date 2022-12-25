/// Get the value of the general configuration when not set in the format and
/// the default one when neither set in the default configuration.
pub mod defaultable;

/// Pandoc arguments structs and enums.
pub mod pandoc_args;

pub use defaultable::*;
pub use pandoc_args::*;
