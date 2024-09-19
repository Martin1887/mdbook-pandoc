/// Headings functions.
mod headings;
/// Internal references and paths functions.
mod links;
/// Process events handled by pulldown-cmark to transform headings.
mod pulldown_events;
#[cfg(test)]
mod tests;

pub(crate) use headings::*;
pub(crate) use links::*;
pub(crate) use pulldown_events::*;
