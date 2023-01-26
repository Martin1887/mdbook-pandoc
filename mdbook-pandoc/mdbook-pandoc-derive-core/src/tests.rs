use std::path::PathBuf;

use colored_diff;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    pandoc_command_args_derive_core, pandoc_repeated_args_derive_core, pandoc_template_gen_core,
    serde_enum_display_derive_core,
};

#[test]
fn test_single_field_struct() {
    let single_field_struct = quote! {
        pub struct SingleFieldStruct {
            pub field1: Option<String>,
        }
    };

    let expected = quote! {
        impl SingleFieldStruct {
            pub fn args(&self, actual_and_default_cfg: &ActualAndDefaultCfg<&SingleFieldStruct>) -> Vec<String> {
                let mut args = Vec::new();
                let actual_val = actual_or_default!(actual_and_default_cfg, field1);
                if actual_val != <String>::default() {
                    let val_str = actual_val.to_string();
                    // the pandoc argument is the field name in kebab-case
                    let pandoc_arg = stringify!(field1).replace('_', "-");
                    if val_str == "true" || val_str == "" {
                        args.push(format!("--{}", pandoc_arg));
                    } else {
                        args.push(format!("--{}={}", pandoc_arg, val_str));
                    }
                }

                args
            }
        }
    };

    assert_tokens_eq(
        &expected,
        &pandoc_command_args_derive_core(single_field_struct),
    );
}

#[test]
fn test_complex_struct() {
    let complex_struct = quote! {
        pub struct ComplexStruct {
            pub field1: Option<String>,
            pub field2: Option<bool>,
            pub field3: Option<Vec<String>>,
            pub field4: Option<u32>,
        }
    };

    let expected = quote! {
        impl ComplexStruct {
            pub fn args(&self, actual_and_default_cfg: &ActualAndDefaultCfg<&ComplexStruct>) -> Vec<String> {
                let mut args = Vec::new();
                let actual_val = actual_or_default!(actual_and_default_cfg, field1);
                if actual_val != <String>::default() {
                    let val_str = actual_val.to_string();
                    // the pandoc argument is the field name in kebab-case
                    let pandoc_arg = stringify!(field1).replace('_', "-");
                    if val_str == "true" || val_str == "" {
                        args.push(format!("--{}", pandoc_arg));
                    } else {
                        args.push(format!("--{}={}", pandoc_arg, val_str));
                    }
                }

                let actual_val = actual_or_default!(actual_and_default_cfg, field2);
                if actual_val != <bool>::default() {
                    let val_str = actual_val.to_string();
                    // the pandoc argument is the field name in kebab-case
                    let pandoc_arg = stringify!(field2).replace('_', "-");
                    if val_str == "true" || val_str == "" {
                        args.push(format!("--{}", pandoc_arg));
                    } else {
                        args.push(format!("--{}={}", pandoc_arg, val_str));
                    }
                }

                let actual_val = actual_or_default!(actual_and_default_cfg, field3);
                if actual_val != < Vec<String> >::default() {
                    let val_str = actual_val.to_string();
                    // the pandoc argument is the field name in kebab-case
                    let pandoc_arg = stringify!(field3).replace('_', "-");
                    if val_str == "true" || val_str == "" {
                        args.push(format!("--{}", pandoc_arg));
                    } else {
                        args.push(format!("--{}={}", pandoc_arg, val_str));
                    }
                }

                let actual_val = actual_or_default!(actual_and_default_cfg, field4);
                if actual_val != <u32>::default() {
                    let val_str = actual_val.to_string();
                    // the pandoc argument is the field name in kebab-case
                    let pandoc_arg = stringify!(field4).replace('_', "-");
                    if val_str == "true" || val_str == "" {
                        args.push(format!("--{}", pandoc_arg));
                    } else {
                        args.push(format!("--{}={}", pandoc_arg, val_str));
                    }
                }

                args
            }
        }
    };

    assert_tokens_eq(&expected, &pandoc_command_args_derive_core(complex_struct));
}

#[test]
#[should_panic]
fn test_wrong_struct() {
    let wrong_struct = quote! {
        pub struct WrongStruct {
            pub field1: Option<String>,
            pub field2: bool,
            pub field3: Vec<String>,
            pub field4: Option<u32>,
        }
    };
    pandoc_command_args_derive_core(wrong_struct);
}

#[test]
fn test_repeated_arg() {
    let test_struct = quote! {
        pub struct TestStruct {
            pub field1: Option<Vec<String>>,
            pub field2: Option<Vec<String>>,
        }
    };
    let expected = quote! {
        impl TestStruct {
            pub fn args(&self, actual_and_default_cfg: &ActualAndDefaultCfg<&TestStruct>) -> Vec<String> {
                let mut args = Vec::new();
                let actual_val = actual_or_default!(actual_and_default_cfg, field1);
                // the pandoc argument is the field name in kebab-case
                let pandoc_arg = stringify!(field1).replace('_', "-");
                for arg_value in actual_val {
                    args.push(format!("--{}={}", pandoc_arg, arg_value));
                }
                let actual_val = actual_or_default!(actual_and_default_cfg, field2);
                // the pandoc argument is the field name in kebab-case
                let pandoc_arg = stringify!(field2).replace('_', "-");
                for arg_value in actual_val {
                    args.push(format!("--{}={}", pandoc_arg, arg_value));
                }

                args
            }
        }
    };

    assert_tokens_eq(&expected, &pandoc_repeated_args_derive_core(test_struct));
}

#[test]
#[should_panic]
fn test_repeated_wrong_struct() {
    let wrong_struct = quote! {
        pub struct WrongStruct {
            pub field1: Option<Vec<String>>,
            pub field2: Option<Vec<Vec<bool>>>,
        }
    };
    pandoc_repeated_args_derive_core(wrong_struct);
}

#[test]
fn test_serde_enum_display_derive() {
    let test_enum = quote! {
        pub enum TestEnum {
            Variant1,
            Variant2,
            Custom(String),
        }
    };
    let expected = quote! {
        impl Display for TestEnum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Custom(v) => write!(f, "{}", v),
                    _ => {
                        write!(f, "{}", serde_yaml::to_string(self).unwrap().trim())
                    }
                }
            }
        }
    };
    assert_tokens_eq(&expected, &serde_enum_display_derive_core(test_enum));
}

#[test]
fn test_templates_enum() {
    let project_root = PathBuf::from(env!["CARGO_MANIFEST_DIR"]).join("..");
    let templates_toml_path = "assets/tests/templates/templates.toml";
    let templates_toml_absolute_path = project_root.join(templates_toml_path);
    let test_template_path = templates_toml_absolute_path
        .parent()
        .unwrap()
        .join("tex/test.tex")
        .into_os_string()
        .into_string()
        .unwrap();
    let test_enum = quote! {
        #[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, SerdeEnumDisplay)]
        #[serde(rename_all = "snake_case")]
        pub enum PandocTemplate {}
    };
    let expected = quote! {
        #[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, SerdeEnumDisplay)]
        #[serde(rename_all = "snake_case")]
        pub enum PandocTemplate {
            /// Use the Pandoc default (not writing the argument).
            #[serde(rename = "")]
            #[default]
            Default,
            /// In TOML write a custom value with the syntax `{"custom" = "value"}`.
            Custom(String),
            #[doc = "Test template"]
            TexTest
        }

        impl PandocResource for PandocTemplate {
            /// Return a String with the license information of the template.
            fn license(&self) -> Option<&str> {
                match self {
                    PandocTemplate::TexTest => Some("License: MIT ().\nRepository URL: "),
                    // Default and custom templates have not a known license
                    _ => None,
                }
            }

            /// Return the description of the template.
            fn description(&self) -> Option<&str> {
                match self {
                    PandocTemplate::TexTest => Some("Test template"),
                    // Default and custom templates have not description
                    _ => None,
                }
            }

            /// Return the contents of the template as a vector of bytes.
            fn contents(&self) -> Option<Vec<u8>> {
                match self {
                    PandocTemplate::TexTest => Some(include_bytes!(#test_template_path).to_vec()),
                    // Default and custom templates have not contents
                    // (custom templates files must already exist in the
                    // Pandoc templates directory)
                    _ => None,
                }
            }

            /// Return the filename that must have the template in the Pandoc
            /// templates directory.
            fn filename(&self) -> Option<&str> {
                match self {
                    PandocTemplate::TexTest => Some("test.tex"),
                    // For custom templates the filename is the specified one
                    PandocTemplate::Custom(s) => Some(s),
                    // Default templates have not a filename
                    _ => None,
                }
            }
        }

        impl Display for PandocTemplate {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.filename() {
                    Some(filename) => write!(f, "{}", filename),
                    None => write!(f, "")
                }
            }
        }
    };
    assert_tokens_eq(
        &expected,
        &pandoc_template_gen_core(quote!(#templates_toml_path), test_enum),
    );
}

// Function really inspired by (`anyinput` crate)
// https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff
fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        println!("expected: {}\n", &expected);
        println!("actual  : {}\n", &actual);
        panic!("expected != actual");
    }
}
