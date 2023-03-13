use std::{
    fs::{self, create_dir},
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    config::{formats::*, resources::*, GeneralConfig},
    write_arg_to_src_folder, write_vec_to_src_folder,
};

use super::*;
use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::actual_or_default;

#[derive(Serialize, Deserialize)]
/// Generic command that contains a format of a specific type and the shared
/// parameters.
pub struct PandocCommand {
    /// Output format.
    pub format: PandocFormat,
    #[serde(flatten)]
    pub generic_args: PandocCommandSharedParameters,
}

impl PandocCommand {
    /// Pandoc command args reading the configuration file.
    /// The `pandoc` command name and the input file are not included.
    pub fn args(
        &self,
        dest_dir: &Path,
        extension: &str,
        general_cfg: &GeneralConfig,
    ) -> Vec<String> {
        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args),
            default: Box::new(&general_cfg.formats_default),
        };
        let mut args = Vec::new();
        let format = self.format.to_string();
        let filename = actual_or_default!(combined_cfg, filename);
        args.push("--self-contained".to_string());
        args.push("--eol=lf".to_string());
        args.push(format!("--to={}", format));
        args.push(format!(
            "--output={}.{}",
            dest_dir.join(filename.to_string()).to_str().unwrap(),
            extension
        ));

        let extra = actual_or_default!(combined_cfg, extra_args);
        args.extend(extra);

        let mut from_format = format!("--from={}", general_cfg.from_format);
        let added_exts = actual_or_default!(combined_cfg, added_extensions);
        if !added_exts.is_empty() {
            from_format.push_str(&format!("+{}", added_exts.join("+")));
        }
        let removed_exts = actual_or_default!(combined_cfg, removed_extensions);
        if !removed_exts.is_empty() {
            from_format.push_str(&format!("-{}", removed_exts.join("-")));
        }
        args.push(from_format);

        let number_sections = actual_or_default!(combined_cfg, number_sections);
        if let DefaultTrueBool::Value(true) = number_sections {
            args.push("--number-sections".to_string())
        };
        let toc = actual_or_default!(combined_cfg, toc);
        if let DefaultTrueBool::Value(true) = toc {
            args.push("--toc".to_string())
        };

        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args.repeated_args),
            default: Box::new(&general_cfg.formats_default.repeated_args),
        };
        args.extend(self.generic_args.repeated_args.args(&combined_cfg));

        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args.args),
            default: Box::new(&general_cfg.formats_default.args),
        };
        args.extend(self.generic_args.args.args(&combined_cfg));

        args
    }

    /// Return the command to be executed using the provided configuration.
    pub fn command(
        &self,
        dest_dir: &Path,
        parsed_md_path: &Path,
        extension: &str,
        general_cfg: &GeneralConfig,
    ) -> Command {
        let mut command = Command::new("pandoc");
        command.args(self.args(dest_dir, extension, general_cfg));
        command.arg(parsed_md_path);

        command
    }

    /// Write templates and assets files.
    pub fn write_template_and_assets_files(&self, general_cfg: &GeneralConfig) {
        self.write_template_file(general_cfg);
        self.write_assets_files(general_cfg);
        self.write_repeated_fields_files(general_cfg);
        self.write_pandoc_fields_files(general_cfg);
    }

    /// Write the files of Pandoc single-time fields to the `src` folder.
    fn write_pandoc_fields_files(&self, general_cfg: &GeneralConfig) {
        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args.args),
            default: Box::new(&general_cfg.formats_default.args),
        };
        write_arg_to_src_folder!(combined_cfg, csl);
        write_arg_to_src_folder!(combined_cfg, reference_doc);
    }

    /// Write the files of repeated fields in the `src` folder.
    fn write_repeated_fields_files(&self, general_cfg: &GeneralConfig) {
        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args.repeated_args),
            default: Box::new(&general_cfg.formats_default.repeated_args),
        };
        write_vec_to_src_folder!(combined_cfg, syntax_definition);
        write_vec_to_src_folder!(combined_cfg, include_in_header);
        write_vec_to_src_folder!(combined_cfg, include_before_body);
        write_vec_to_src_folder!(combined_cfg, include_after_body);
        write_vec_to_src_folder!(combined_cfg, css);
        write_vec_to_src_folder!(combined_cfg, epub_embed_font);

        for filter in actual_or_default!(combined_cfg, filter) {
            match filter {
                PandocFilter::Default | PandocFilter::Custom(_) => {}
                _ => {
                    self.write_data_dir_file(
                        general_cfg,
                        "filters",
                        filter
                            .filename()
                            .expect("All filters must have filename except default."),
                        &filter
                            .contents()
                            .expect("All filters must return contents except default and custom"),
                    );
                }
            }
        }
        for filter in actual_or_default!(combined_cfg, lua_filter) {
            match filter {
                PandocLuaFilter::Default | PandocLuaFilter::Custom(_) => {}
                _ => {
                    self.write_data_dir_file(
                        general_cfg,
                        "filters",
                        filter
                            .filename()
                            .expect("All filters must have filename except default."),
                        &filter
                            .contents()
                            .expect("All filters must return contents except default and custom"),
                    );
                }
            }
        }
    }

    /// Write the assets files in the `src` folder.
    fn write_assets_files(&self, general_cfg: &GeneralConfig) {
        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args),
            default: Box::new(&general_cfg.formats_default),
        };
        write_vec_to_src_folder!(combined_cfg, assets);
    }

    /// Write the template file.
    fn write_template_file(&self, general_cfg: &GeneralConfig) {
        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args.args),
            default: Box::new(&general_cfg.formats_default.args),
        };
        let template = actual_or_default!(combined_cfg, template);
        match template {
            PandocTemplate::Default | PandocTemplate::Custom(_) => {}
            _ => {
                self.write_data_dir_file(
                    general_cfg,
                    "templates",
                    template
                        .filename()
                        .expect("All templates must have filename except default."),
                    &template
                        .contents()
                        .expect("All templates must return contents except default and custom"),
                );
            }
        }
    }

    /// Write the template file if the template is not default nor custom.
    fn write_data_dir_file(
        &self,
        general_cfg: &GeneralConfig,
        subfolder: &str,
        filename: &str,
        contents: &[u8],
    ) {
        let templates_dir = self.get_pandoc_actual_data_dir(general_cfg).join(subfolder);
        if !templates_dir.is_dir() {
            create_dir(&templates_dir)
                .expect("Error creating the templates directory in the data-dir");
        }
        fs::write(templates_dir.join(filename), contents)
            .expect("Error writing the contents of the template");
    }

    /// Determines the actual data directory reading the value of the custom
    /// one and the default one.
    fn get_pandoc_actual_data_dir(&self, general_cfg: &GeneralConfig) -> PathBuf {
        let combined_cfg = ActualAndDefaultCfg {
            actual: Box::new(&self.generic_args.args),
            default: Box::new(&general_cfg.formats_default.args),
        };
        let custom_data_dir = actual_or_default!(combined_cfg, data_dir);
        if custom_data_dir.is_empty() {
            let default_data_dir = Self::get_pandoc_default_data_dir().unwrap();
            Path::new(&default_data_dir).to_path_buf()
        } else {
            Path::new(&custom_data_dir).to_path_buf()
        }
    }

    /// Determine the Pandoc default data directory checking the possible default
    /// directories specified in its documentation.
    pub fn get_pandoc_default_data_dir() -> Result<String, Error> {
        if cfg!(windows) {
            let username_var = std::env::var("USERNAME");
            if let Ok(username) = username_var {
                let data_dir = Path::new(r"C:\Users")
                    .join(username)
                    .join(r"AppData\Roaming\pandoc");
                if data_dir.is_dir() {
                    Ok(data_dir.to_str().unwrap().to_string())
                } else {
                    Err(Error::msg(
                        r"C:\Users\USERNAME\AppData\Roaming\pandoc does not exist.",
                    ))
                }
            } else {
                Err(Error::msg(
                    "Error getting the default data directory, USERNAME is not defined.",
                ))
            }
        } else {
            let home_var = std::env::var("HOME");
            if let Ok(home) = home_var {
                let data_dir = Path::new(&home).join(".local/share/pandoc");
                let legacy_data_dir = Path::new(&home).join(".pandoc");
                if data_dir.is_dir() {
                    Ok(data_dir.to_str().unwrap().to_string())
                } else if legacy_data_dir.is_dir() {
                    Ok(legacy_data_dir.to_str().unwrap().to_string())
                } else {
                    Err(Error::msg(
                        "Nor $HOME/.local/share/pandoc nor $HOME/.pandoc exist.",
                    ))
                }
            } else {
                Err(Error::msg(
                    "Error getting the Pandoc default data directory, $HOME is not defined.",
                ))
            }
        }
    }
}
