// clean.rs
//
// responsible for cleaning tasks

use crate::misc::static_exec;
use crate::package::Package;
use crate::paths::SOURCES;
use crate::{erm, vpr};
use std::fs;

pub fn prune_sources(p: &Package) -> u8 {
    let kept = format!("{}-{}.tar", p.name, p.version);
    let mut num_removed: u8 = 0;

    if let Ok(entries) = fs::read_dir(&*SOURCES) {
        for entry in entries.filter_map(Result::ok) {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if entry.file_type().map_or(false, |t| t.is_file())
                && (file_name_str.starts_with(&p.name) || file_name_str.starts_with(&p.name.replace("_", "-")))
                && file_name_str != kept
            {
                match fs::remove_file(entry.path()) {
                    Ok(_) => {
                        num_removed += 1;
                        vpr!("Removed '{}'", file_name_str);
                    }
                    Err(e) => erm!("Failed to remove file '{}': {}", file_name_str, e),
                }
            }
        }
    } else {
        erm!("Failed to read sources directory");
    }
    num_removed
}

pub fn remove_tarballs(pkg_str: &str) {
    let command = format!("cd {} && rm -vf {}-[0-9]*.t*", SOURCES.display(), pkg_str);
    let _ = static_exec(&command);
}
