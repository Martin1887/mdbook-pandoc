use std::{path::PathBuf, process::Command};

use crate::config::{formats::*, GeneralConfig};

use super::*;
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
        dest_dir: &PathBuf,
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
            dest_dir.join(&filename.to_string()).to_str().unwrap(),
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
        match number_sections {
            DefaultTrueBool::Value(true) => args.push("--number-sections".to_string()),
            _ => {}
        };
        let toc = actual_or_default!(combined_cfg, toc);
        match toc {
            DefaultTrueBool::Value(true) => args.push("--toc".to_string()),
            _ => {}
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
        dest_dir: &PathBuf,
        parsed_md_path: &PathBuf,
        extension: &str,
        general_cfg: &GeneralConfig,
    ) -> Command {
        let mut command = Command::new("pandoc");
        command.args(self.args(dest_dir, extension, general_cfg));
        command.arg(parsed_md_path);

        command
    }
}
