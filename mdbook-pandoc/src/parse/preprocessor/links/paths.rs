use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use super::{PATH_RE, URI_PROTOCOL_RE};
use log::warn;
use regex::{Captures, Replacer};

/// Struct with the source path implementing the `Replacer` trait to be able to
/// use the source dir path in the replacement function.
struct SourcePathReplacer {
    source_dir_path: PathBuf,
    book_paths: HashSet<PathBuf>,
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
        let mut path_str = path_match.as_str().to_string();
        let orig_path_str = path_str.clone();
        let mut path = Path::new(&path_str);
        let path_buf;
        // if the path is relative, make it relative to the `src` directory
        if path.is_relative() && !path_str.starts_with('#') && !URI_PROTOCOL_RE.is_match(&path_str)
        {
            path_buf = self.source_dir_path.join(path);
            path = path_buf.as_path();
            path_str = path.to_str().unwrap().to_string();
        }
        // only replace paths that exist, assuming wrong paths are labels or other things
        if path.is_file() {
            replace = true;
            // if the path is MD file of the book, append a `#` to link
            // in a posterior step to the first heading of the file
            if self.book_paths.contains(&path.canonicalize().unwrap()) {
                path_str.push('#');
            }
        } else {
            // the path can contain a link at the end, try to remove it and check again
            match path_str.rfind('#') {
                Some(fragment_pos) => {
                    let clean_path_str = &path_str[0..fragment_pos];
                    let clean_path = PathBuf::from(clean_path_str);
                    if clean_path.is_file() {
                        replace = true;
                    } else {
                        warn_if_not_uri_nor_internal_link(&path_str);
                    }
                }
                None => {
                    warn_if_not_uri_nor_internal_link(&path_str);
                }
            };
        }

        if replace {
            replacement = replacement.replace(&orig_path_str, &path_str);
        }

        dst.push_str(&replacement);
    }
}

/// Print a warning if the path is not a URI nor an internal link.
fn warn_if_not_uri_nor_internal_link(path_str: &str) {
    if !path_str.starts_with('#') && !URI_PROTOCOL_RE.is_match(path_str) {
        warn!("Missing path: `{}`.", path_str);
    }
}

/// Translate relative paths to be relative to the `src` folder.
pub(crate) fn translate_relative_paths(
    formatted: &str,
    source_path: &Option<PathBuf>,
    book_paths: &HashSet<PathBuf>,
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
