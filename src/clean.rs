// clean.rs
//
// responsible for cleaning tasks

use crate::paths::SOURCES;
use crate::pr;
use std::fs;

pub fn sources(pkg_str: String, pkg_ver: String) {
    let kept = format!("{}-{}.tar", pkg_str, pkg_ver);

    if let Ok(entries) = fs::read_dir(&*SOURCES) {
        for entry in entries.filter_map(Result::ok) {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if entry.file_type().map_or(false, |t| t.is_file())
                && file_name_str.starts_with(&pkg_str)
                && file_name_str != kept
            {
                if let Err(e) = fs::remove_file(entry.path()) {
                    eprintln!("Failed to remove file '{}': {}", file_name_str, e);
                } else {
                    pr!(format!("Removed '{}'", file_name_str));
                }
            }
        }
    } else {
        eprintln!("Failed to read sources directory");
    }
}
