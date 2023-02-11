use clap::{Parser, Subcommand, ValueEnum};
use mdbook::{renderer::RenderContext, Renderer};
use mdbook_pandoc::{config::resources::*, PandocRenderer};
use mdbook_pandoc_derive::AssetTypeList;
use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
}

#[derive(Subcommand)]
enum AssetsSubcommand {
    /// List the assets.
    List(ListSubcommand),
    /// Print the contents of a specific asset.
    Print(PrintSubcommand),
}

#[derive(Parser, Clone)]
struct ListSubcommand {
    /// Select the listed asset types, if it is not specified all are selected.
    #[arg(short, long)]
    types: Option<Vec<AssetType>>,
    /// Select the format in which display the values, `plain` by default.
    #[arg(short, long)]
    format: Option<ListFormat>,
}

#[derive(Clone, ValueEnum, EnumIter, AssetTypeList)]
enum AssetType {
    Template,
    SyntaxDefinition,
    IncludeInHeader,
    IncludeBeforeBody,
    IncludeAfterBody,
    Css,
    Csl,
    ReferenceDoc,
    TemplateAsset,
    Filter,
    LuaFilter,
}

trait AssetTypeList {
    fn to_plain(&self) -> String;
    fn to_json(&self) -> String;
    fn to_yaml(&self) -> String;
}

#[derive(Clone, ValueEnum, Default)]
enum ListFormat {
    #[default]
    Plain,
    Json,
    Yaml,
}

trait AssetPrint {
    fn print(&self) -> Vec<u8>;
}

// Try to serialize the enum, calling the
// recursively the macro if it fails, and
// panic if all serializations fail
macro_rules! serialize_and_contents {
    ($name: expr, [$head: ident, $($tail: ident),*]) => {
        match serde_yaml::from_str::<$head>($name) {
            Ok(object) => {
                object.contents()
                .expect("Specify a built-in asset variant, not `Custom` nor `Default`")
            },
            _ => {
                serialize_and_contents!($name, [$($tail),*])
            }
        }
    };
    ($name: expr, [$head: ident]) => {
        serde_yaml::from_str::<$head>($name)
            .expect("Specify a built-in asset variant, not `Custom` nor `Default`")
            .contents()
            .expect("Specify a built-in asset variant, not `Custom` nor `Default`")
    };
}

#[derive(Parser, Clone)]
struct PrintSubcommand {
    /// The asset name.
    asset_name: String,
}
impl AssetPrint for PrintSubcommand {
    fn print(&self) -> Vec<u8> {
        serialize_and_contents!(
            &self.asset_name,
            [
                PandocTemplate,
                PandocSyntaxDefinition,
                PandocIncludeInHeader,
                PandocIncludeBeforeBody,
                PandocIncludeAfterBody,
                PandocCss,
                PandocCsl,
                PandocReferenceDoc,
                PandocTemplateAsset,
                PandocFilter,
                PandocLuaFilter
            ]
        )
    }
}

fn list_assets(types: &[AssetType], format: &ListFormat) -> String {
    let assets: Vec<String> = types
        .iter()
        .map(|el| match format {
            ListFormat::Plain => el.to_plain(),
            ListFormat::Json => el.to_json(),
            ListFormat::Yaml => el.to_yaml(),
        })
        .collect();
    match format {
        ListFormat::Plain => assets.join("\n"),
        ListFormat::Json => format!("[{}]", assets.join(",\n")),
        ListFormat::Yaml => assets.join("\n"),
    }
}

/// The main function that parses the book and generates outputs.
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
        _ => {
            let mut stdin = io::stdin();
            let ctx = RenderContext::from_json(&mut stdin).unwrap();
            PandocRenderer
                .render(&ctx)
                .expect("Error building the book with the pandoc renderer");
        }
    }
}

mod tests {
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        crate::Args::command().debug_assert()
    }
}
