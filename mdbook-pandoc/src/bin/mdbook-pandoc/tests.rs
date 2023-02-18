use std::fs::{read, remove_file, File};

use crate::configs::write_in_book_config;

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    crate::Args::command().debug_assert()
}

#[test]
fn test_write_in_book_config() {
    let initial_contents = r#"
[output.pandoc.general]
unlist_not_main_headers = false
"#;
    let destination_file = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/tests/test_config_load.toml"
    )
    .to_string();
    File::create(&destination_file).unwrap();

    write_in_book_config(
        initial_contents.as_bytes(),
        Some(destination_file.clone()),
        true,
    )
    .unwrap();
    assert_eq!(
        String::from_utf8_lossy(&read(&destination_file).unwrap()).trim(),
        initial_contents.trim(),
        "Contents not written in the configuration file"
    );

    let new_contents = r#"
[output.pandoc.not_general]
preamble_label = "My custom preamble"
    "#
    .as_bytes();

    write_in_book_config(new_contents, Some(destination_file.clone()), false).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&read(&destination_file).unwrap()).trim(),
        r#"
[output.pandoc.general]
unlist_not_main_headers = false

[output.pandoc.not_general]
preamble_label = "My custom preamble"
"#
        .trim(),
        "Configuration files have not been correctly merged"
    );

    write_in_book_config(new_contents, Some(destination_file.clone()), true).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&read(&destination_file).unwrap()).trim(),
        r#"

[output.pandoc.general]


[output.pandoc.not_general]
preamble_label = "My custom preamble"
"#
        .trim(),
        "Configuration files have not been correctly merged"
    );

    remove_file(&destination_file).unwrap();
}
