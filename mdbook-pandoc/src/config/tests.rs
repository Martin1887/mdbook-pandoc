//! Tests for the configuration parameters

use std::fs;

use mdbook::Config;

use crate::metadata::{fields::MetadataTitle, Metadata};

use super::metadata::MetadataConfig;

#[test]
fn test_metadata_from_path() {
    let cfg_toml = r#"
        [output.pandoc]
        metadata = "metadata.yml"
    "#;
    let toml: Config = toml::from_str(&cfg_toml).expect("Error loading test TOML");

    match toml
        .get_deserialized_opt("output.pandoc.metadata")
        .expect("Error getting metadata value")
    {
        Some(MetadataConfig::Path(path)) => {
            let metadata_path = format!(
                "{}/{}/{}",
                env!("CARGO_MANIFEST_DIR"),
                "assets/tests/parse_test",
                path
            );
            let metadata: Metadata = serde_yaml::from_str(
                &fs::read_to_string(&metadata_path).expect("Error reading the metadata file"),
            )
            .expect("Error parsing metadata from file");
            match metadata.title.expect("No title in metadata") {
                MetadataTitle::Text(title) => assert_eq!(title, String::from("My nice title")),
                _ => panic!("No text title in metadata"),
            };
            let ibooks = metadata.ibooks.expect("No ibooks in metadata");
            assert_eq!(ibooks.binding, None);
            assert_eq!(ibooks.version, "4.0");
            assert_eq!(metadata.publisher, None);
        }
        Some(_) => panic!("Metadata path not read as path"),
        None => panic!("Error getting metadata value"),
    }
}
