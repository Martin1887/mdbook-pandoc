use serde_yaml;
use toml;

use crate::metadata::Metadata;

fn get_basic_metadata_toml() -> String {
    r#"
    title = "Test ebook"
    description = "This description is a test"
    "#
    .to_string()
}

fn get_complex_metadata_toml() -> String {
    r#"
    title = "Test ebook"
    creator = [{role = "author", text = "Test author"}, {role = "illustrator", text = "Test"}]
    contributor = "My friends"
    type = "Free"
    coverage = "Full"
    rights = "© My and my friends"
    date = "2022-01-01"
    publisher = "Test publisher"
    lang = "en-US"
    description = "This description is a test"
    group-position = 0
    ibooks = {version = "1.0", ipad-orientation-lock = "portrait-only", specified-fonts = false, binding = true, scroll-axis = "horizontal"}
    "#
    .to_string()
}

#[test]
fn test_basic_metadata() {
    let input_text = get_basic_metadata_toml();
    let parsed: Metadata = toml::from_str(&input_text).unwrap();
    assert_eq!(
        "title: Test ebook\ndescription: This description is a test\n".to_string(),
        format!("{}", serde_yaml::to_string(&parsed).unwrap())
    );
}

#[test]
fn test_complex_metadata() {
    let input_text = get_complex_metadata_toml();
    let correct_output = r#"title: Test ebook
creator:
- role: author
  text: Test author
- role: illustrator
  text: Test
contributor: My friends
date: 2022-01-01
lang: en-US
description: This description is a test
type: Free
coverage: Full
rights: © My and my friends
group-position: 0
ibooks:
  version: '1.0'
  specified-fonts: false
  ipad-orientation-lock: portrait-only
  iphone-orientation-lock: null
  binding: true
  scroll-axis: horizontal
"#;
    let parsed: Metadata = toml::from_str(&input_text).unwrap();
    assert_eq!(
        correct_output,
        format!("{}", serde_yaml::to_string(&parsed).unwrap())
    );
}
