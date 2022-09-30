use mdbook::renderer::RenderContext;
use mdbook_pandoc::run;
use std::io;

/// The main function that parses the book and generates outputs.
fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    run(&ctx);
}
