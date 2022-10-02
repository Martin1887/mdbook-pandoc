use mdbook::{renderer::RenderContext, Renderer};
use mdbook_pandoc::PandocRenderer;
use std::io;

/// The main function that parses the book and generates outputs.
fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    PandocRenderer
        .render(&ctx)
        .expect("Error building the book with the pandoc renderer");
}
