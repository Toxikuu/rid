// clean.rs
//
// responsible for cleaning tasks

use crate::misc::static_exec;
use crate::package::Package;
use crate::paths::SOURCES;
use crate::{erm, vpr};
use std::fs;

pub fn prune_sources(p: &Package) {
    let kept = format!("{}-{}.tar", p.name, p.version);

    if let Ok(entries) = fs::read_dir(&*SOURCES) {
        for entry in entries.filter_map(Result::ok) {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if entry.file_type().map_or(false, |t| t.is_file())
                && file_name_str.starts_with(&p.name)
                && file_name_str != kept
            {
                if let Err(e) = fs::remove_file(entry.path()) {
                    erm!("Failed to remove file '{}': {}", file_name_str, e);
                } else {
                    vpr!("Removed '{}'", file_name_str);
                }
            }
        }
    } else {
        erm!("Failed to read sources directory");
    }
}

pub fn remove_tarballs(pkg_str: &str) {
    let command = format!("cd /etc/rid/sources && rm -vf {}-[0-9]*.t*", pkg_str);
    let _ = static_exec(&command);
}
