use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use super::PATH_RE;
use regex::{Captures, Replacer};

/// Struct with the source path implementing the `Replacer` trait to be able to
/// use the source dir path in the replacement function.
struct SourcePathReplacer {
    source_dir_path: PathBuf,
    book_paths: Box<HashSet<PathBuf>>,
}

impl Replacer for SourcePathReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        // The replacement is doing nothing unless below conditions apply,
        // which replace the original path by the concatenation of the source
        // and original path
        let mut replace = false;
        let mut replacement = String::from(caps.get(0).unwrap().as_str());

        // get the match (the path of a reference or a link)
        let path_match = caps.name("ref_path").unwrap_or_else(|| {
            caps.name("path")
                .expect("No capture with name `ref_path` nor `path` in the regex")
        });
        let path_str = path_match.as_str();
        let path = Path::new(&path_str);
        if path.is_relative() {
            let new_path = self.source_dir_path.join(path);
            let mut new_path_str = new_path.to_str().unwrap().to_string();
            // only replace paths that exist, assuming wrong paths are labels or other things
            if new_path.is_file() {
                replace = true;
                // if the path is MD file of the book, append a `#` to link
                // in a posterior step to the first header of the file
                if self.book_paths.contains(&new_path.canonicalize().unwrap()) {
                    new_path_str.push_str("#");
                }
            } else if new_path_str.contains('#') {
                // the path can contain a link at the end, try to remove it and check again
                match new_path_str.rfind('#') {
                    Some(fragment_pos) => {
                        let clean_path_str = &new_path_str[0..fragment_pos];
                        let clean_path = PathBuf::from(clean_path_str);
                        if clean_path.is_file() {
                            replace = true;
                        }
                    }
                    None => {}
                };
            }

            if replace {
                replacement = replacement.replace(path_str, &new_path_str);
            }
        }

        dst.push_str(&replacement);
    }
}

/// Translate relative paths to be relative to the `src` folder.
pub(crate) fn translate_relative_paths(
    formatted: &str,
    source_path: &Option<PathBuf>,
    book_paths: &Box<HashSet<PathBuf>>,
) -> String {
    let source_dir_path = match source_path {
        // the path is a file, so its directory must be chosen
        Some(ref path) => path
            .clone()
            .parent()
            .expect("Error getting the directory of the path")
            .to_path_buf(),
        None => panic!("Error getting the source path of a chapter file"),
    };
    let replacer = SourcePathReplacer {
        source_dir_path,
        book_paths: book_paths.clone(),
    };

    PATH_RE.replace_all(formatted, replacer).to_string()
}
