// utils.rs
//
// responsible for defining utility functions

use crate::package::{Package, PackageStatus};
use crate::{die, vpr, msg};
use crate::flags::FORCE;
use std::io;
use std::collections::HashSet;
use std::fs;
use std::time::SystemTime;
use std::path::Path;

pub fn mkdir(path: &Path) {
    if path.exists() {
        return;
    }

    if let Err(e) = fs::create_dir_all(path) {
        die!("Failed to create '{}': {}", path.display(), e)
    }

    vpr!("Created directory '{}'", path.display());
}

pub fn get_mod_time(path: &Path) -> io::Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    metadata
        .modified()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn format_line(line: &str, max_length: usize) -> String {
    let parts: Vec<&str> = line.split('~').collect();

    if parts.len() < 2 {
        return line.to_string();
    }

    let package_info = parts[0].trim();
    let status = parts[1].trim();
    let formatted_status = if status.contains("Available") {
        format!("\x1b[30m{}\x1b[0m", status)
    } else if status.contains("Installed") {
        format!("\x1b[36;1m{}\x1b[0m", status)
    } else {
        unreachable!("Invalid status for format_line()")
    };

    let name_version_length = package_info.len() + 1;
    let padding = max_length.saturating_sub(name_version_length);
    let spaces = " ".repeat(padding);

    format!("{}{} ~ {}", package_info, spaces, formatted_status)
}

pub fn display_list(list: &[Package] ) {
    list.to_owned().sort_by(|a, b| a.name.cmp(&b.name)); // i love you rust analyzer

    for p in list.iter() {
        let line = format!(
            "{}={} ~ {:?} {}",
            p.name, p.version, p.status, p.installed_version
        );
        let formatted_line = format_line(&line, 32);
        println!("  {}", formatted_line);
    }

    vpr!("Displayed {} packages", list.len())
}

pub fn dedup(mut vec: Vec<Package>) -> Vec<Package> {
    vec.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    vec.dedup_by(|a, b| a.name == b.name);
    vec
}

pub fn do_install(p: &Package) -> bool {
    match p.status {
        PackageStatus::Installed => {
            msg!("{} is already installed", p);
            *FORCE.lock().unwrap()
        }
        _ => {
            msg!("Installing {}", p);
            true
        }
    }
}

pub fn read_dir_recursive(path: &Path, json_mod_time: SystemTime, cache_list: &mut Vec<String>, ignored: &HashSet<String>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if ignored.contains(path.file_name().unwrap_or_default().to_str().unwrap_or_default()) {
            continue;
        }

        if path.is_dir() {
            read_dir_recursive(&path, json_mod_time, cache_list, ignored)?;
        } else if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            let entry_mod_time = get_mod_time(&path)?;

            if entry_mod_time >= json_mod_time {
                vpr!("Caching package '{}'...", pkg_str);
                cache_list.push(pkg_str.to_string());
            }
        }
    }
    Ok(())
}
