// tracking.rs
//
// responsible for keeping track of packages

use crate::checks::is_file_empty;
use crate::misc::get_mod_time;
use crate::package::{form_package, Package, PackageStatus};
use crate::paths::{FAILED, META, PKGSJSON};
use crate::{die, vpr};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::{from_str, to_string_pretty};
use std::collections::HashSet;
use std::fs::{self, read_to_string, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn create_json() -> io::Result<()> {
    if !is_file_empty(&PKGSJSON) {
        return Ok(());
    }

    let mut file = File::create(&*PKGSJSON)?;
    file.write_all(b"[]")?;
    vpr!("Wrote [] to empty pkgs.json");

    Ok(())
}

pub fn read_pkgs_json() -> Result<Vec<Package>, String> {
    let contents = read_to_string(&*PKGSJSON).map_err(|e| e.to_string())?;
    from_str(&contents).map_err(|e| e.to_string())
}

pub fn load_package_list() -> Vec<Package> {
    let mut file = File::open(&*PKGSJSON).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    let pkg_list: Vec<Package> = from_str(&content).unwrap();
    pkg_list
}

pub fn save_package_list(pkg_list: &Vec<Package>) {
    let jsdata = to_string_pretty(pkg_list).expect("Failed to serialize package data");
    let mut file = File::create(&*PKGSJSON).expect("Failed to create $RIDPKGSJSON");
    file.write_all(jsdata.as_bytes())
        .expect("Failed to write to $RIDPKGSJSON");
}

fn build_failed() -> bool {
    Path::new(&*FAILED).exists()
}

pub fn add_package(pkg_list: &mut Vec<Package>, p: &Package) -> Result<(), String> {
    if build_failed() {
        return Err("Not tracking due to build failure".to_string());
    }

    if let Some(package) = pkg_list.iter_mut().find(|pkg| pkg.name == p.name) {
        vpr!("Adding package: '{}'", package.name);
        package.status = PackageStatus::Installed;
        package.installed_version = package.version.clone();
    }

    save_package_list(pkg_list);
    Ok(())
}

pub fn remove_package(pkg_list: &mut Vec<Package>, pkg_name: &str) -> Result<(), String> {
    if let Some(package) = pkg_list.iter_mut().find(|p| p.name == pkg_name) {
        package.status = PackageStatus::Available;
        package.installed_version = "".to_string();
        save_package_list(pkg_list);
        Ok(())
    } else {
        Err(format!("Package '{}' not found", pkg_name))
    }
}

const TEMPLATE: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {pos}/{len} ({eta})";

pub fn cache_changes(pkg_list: &mut Vec<Package>, cache_all: bool) -> io::Result<u16> {
    // caches changes made in $RIDMETA to $RIDPKGSJSON
    let json_mod_time = get_mod_time(&PKGSJSON)?;
    let ignored: HashSet<String> = ["README.md", "LICENSE", ".git"]
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let mut cache_list: Vec<String> = Vec::new();

    for entry in fs::read_dir(&*META)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            if !cache_all {
                let entry_mod_time = get_mod_time(&path)?;

                if entry_mod_time >= json_mod_time {
                    vpr!("Caching package '{}'...", pkg_str);
                    cache_list.push(pkg_str.to_string());
                }
            } else {
                vpr!("Caching package '{}'...", pkg_str);
                cache_list.push(pkg_str.to_string());
            }
        }
    }

    cache_list.retain(|item| !ignored.contains(item));
    if cache_list.is_empty() {
        return Ok(0);
    }

    let length = cache_list.len() as u64;
    let bar = ProgressBar::new(length);

    bar.set_message("Caching packages...");
    bar.set_style(
        ProgressStyle::with_template(TEMPLATE)
            .unwrap()
            .progress_chars("#|-"),
    );
    bar.set_length(length);

    for pkg_str in cache_list {
        match form_package(&pkg_str) {
            Ok(pkg) => {
                if let Some(pos) = pkg_list.iter_mut().position(|p| p.name == pkg.name) {
                    pkg_list[pos] = pkg;
                    bar.inc(1);
                } else {
                    pkg_list.push(pkg);
                    bar.inc(1);
                }

                if length > 0 {
                    save_package_list(pkg_list);
                }
            }
            Err(e) if e == "refused" => continue,
            Err(e) => die!("Error processing '{}': {}", pkg_str, e),
        }
    }

    bar.finish_with_message("Cached!");
    Ok(length as u16)
}
