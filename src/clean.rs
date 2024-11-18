// clean.rs
//
// responsible for cleaning tasks

use crate::misc::static_exec;
use crate::package::Package;
use crate::paths::SOURCES;
use crate::{die, erm, vpr};
use std::fs;

fn is_removable(entry: &fs::DirEntry, p: &Package) -> bool {
    let kept = format!("{}-{}.tar", p.name, p.version);
    let file_name = entry.file_name();
    let file_name_str = file_name.to_string_lossy();
    entry.file_type().is_ok_and(|t| t.is_file())
        && file_name_str.starts_with(&p.name)
        && file_name_str != kept
}

fn remove_file(entry: &fs::DirEntry) -> Result<(), std::io::Error> {
    let file_name = entry.file_name();
    if let Err(e) = fs::remove_file(entry.path()) {
        erm!("Failed to remove file '{:?}': {}", file_name, e);
        return Err(e);
    }
    Ok(())
}

pub fn prune_sources(p: &Package) -> u8 {
    let mut num_removed: u8 = 0;

    if let Err(e) = fs::read_dir(&*SOURCES).map(|entries| {
        entries
            .filter_map(Result::ok)
            .filter(|entry| is_removable(entry, p))
            .for_each(|entry| {
                if remove_file(&entry).is_ok() {
                    num_removed += 1;
                    vpr!("Removed {:?}", entry);
                }
            })
    }) {
        die!("Failed to read sources directory: {}", e);
    }

    num_removed
}

pub fn remove_tarballs(pkg_str: &str) {
    let command = format!("cd {} && rm -vf {}-[0-9]*.t*", SOURCES.display(), pkg_str);
    if let Err(e) = static_exec(&command) {
        erm!("Failed to remove tarballs for '{}': {}", pkg_str, e)
    }
}
