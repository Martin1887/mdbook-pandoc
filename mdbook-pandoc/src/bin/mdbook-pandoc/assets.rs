use clap::{Parser, Subcommand, ValueEnum};
use mdbook_pandoc::config::resources::*;
use mdbook_pandoc_derive::AssetTypeList;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Subcommand)]
pub(crate) enum AssetsSubcommand {
    /// List the assets.
    List(ListSubcommand),
    /// Print the contents of a specific asset.
    Print(PrintSubcommand),
}

#[derive(Parser, Clone)]
pub(crate) struct ListSubcommand {
    /// Select the listed asset types, if it is not specified all are selected.
    #[arg(short, long)]
    pub(crate) types: Option<Vec<AssetType>>,
    /// Select the format in which display the values, `plain` by default.
    #[arg(short, long)]
    pub(crate) format: Option<ListFormat>,
}

#[derive(Clone, ValueEnum, EnumIter, AssetTypeList)]
pub(crate) enum AssetType {
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

pub(crate) trait AssetTypeList {
    fn to_plain(&self) -> String;
    fn to_json(&self) -> String;
    fn to_yaml(&self) -> String;
}

#[derive(Clone, ValueEnum, Default)]
pub(crate) enum ListFormat {
    #[default]
    Plain,
    Json,
    Yaml,
}

pub(crate) fn list_assets(types: &[AssetType], format: &ListFormat) -> String {
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

pub(crate) trait AssetPrint {
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
        match serde_yaml::from_str::<$head>($name) {
            Ok(object) => {
                object.contents()
                    .expect("Specify a built-in asset variant, not `Custom` nor `Default`")
            },
            _ => {
                panic!("Specify a built-in asset variant")
            }
        }
    };
}

#[derive(Parser, Clone)]
pub(crate) struct PrintSubcommand {
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
