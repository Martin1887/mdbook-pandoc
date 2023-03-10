pub(crate) mod assets;
pub(crate) mod configs;
#[cfg(test)]
mod tests;

pub(crate) use assets::*;
pub(crate) use configs::*;

use clap::{Parser, Subcommand};
use mdbook::{renderer::RenderContext, Renderer};
use mdbook_pandoc::config::TomlLoad;
use mdbook_pandoc::PandocRenderer;
use strum::IntoEnumIterator;

use std::fs::read;
use std::io;

/// mdbook-pandoc arguments and subcommands.
#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand, Default)]
enum Subcommands {
    /// Run the renderer, the default option if no subcommand is specified.
    #[default]
    Render,
    /// List of print assets.
    Assets {
        #[command(subcommand)]
        command: AssetsSubcommand,
    },
    /// Load or save config.
    Configs {
        #[command(subcommand)]
        command: ConfigsSubcommand,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Subcommands::Assets { command }) => match command {
            AssetsSubcommand::List(list_command) => {
                let types = list_command.types.unwrap_or(AssetType::iter().collect());
                let format = list_command.format.unwrap_or_default();
                println!("{}", list_assets(&types, &format));
            }
            AssetsSubcommand::Print(print_command) => {
                println!("{}", String::from_utf8_lossy(&print_command.print()));
            }
        },
        Some(Subcommands::Configs { command }) => match command {
            ConfigsSubcommand::Load {
                config_to_load,
                destination_file,
                clear,
            } => write_in_book_config(&config_to_load.load(), destination_file, clear).unwrap(),
            ConfigsSubcommand::LoadFile {
                config_path,
                destination_file,
                clear,
            } => {
                write_in_book_config(&read(config_path).unwrap(), destination_file, clear).unwrap()
            }
            ConfigsSubcommand::List => {
                println!("{}", list_configs());
            }
            ConfigsSubcommand::Print { config } => {
                println!("{}", String::from_utf8_lossy(&config.load()));
            }
        },
        _ => {
            let mut stdin = io::stdin();
            let ctx = RenderContext::from_json(&mut stdin).unwrap();
            PandocRenderer
                .render(&ctx)
                .expect("Error building the book with the pandoc renderer");
        }
    }
}
