//! Configuration structs and parsing.

pub(crate) mod metadata;
#[cfg(test)]
mod tests;

/// Labels for the titles.
pub(crate) struct TitleLabels {
    pub preamble: String,
    pub chapters: String,
    pub annexes: String,
}
