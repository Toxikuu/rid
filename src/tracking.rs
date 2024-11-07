// tracking.rs
//
// responsible for keeping track of packages

use crate::{erm, die};
use crate::package::{form_package, Package, PackageStatus};
use crate::paths::{META, PKGSJSON, FAILED};
use serde_json::{from_str, to_string_pretty};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn load_package_list(file_path: &Path) -> io::Result<Vec<Package>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let pkg_list: Vec<Package> = from_str(&content)?;
    Ok(pkg_list)
}

pub fn save_package_list(pkg_list: &Vec<Package>) {
    let jsdata = to_string_pretty(pkg_list).expect("Failed to serialize package data");
    let mut file = File::create(&*PKGSJSON).expect("Failed to create pkgs.json");
    file.write_all(jsdata.as_bytes()).expect("Failed to write to pkgs.json");
}

fn build_failed() -> bool {
    Path::new(&*FAILED).exists()
}

pub fn add_package(pkg_list: &mut Vec<Package>, p: &Package) -> Result<(), String> {
    if build_failed() {
        return Err("Not tracking due to build failure".to_string());
    }

    if let Some(existing_pkg) = pkg_list.iter_mut().find(|pkg| pkg.name == p.name) {
        existing_pkg.status = PackageStatus::Installed;
    } else {
        let mut new_pkg = p.clone();
        new_pkg.status = PackageStatus::Installed;
        pkg_list.push(new_pkg);
    }

    save_package_list(pkg_list);
    Ok(())
}

pub fn remove_package(pkg_list: &mut Vec<Package>, pkg_name: &str) -> Result<(), String> {
    if let Some(package) = pkg_list.iter_mut().find(|p| p.name == pkg_name) {
        package.status = PackageStatus::Available;
        save_package_list(pkg_list);
        Ok(())
    } else {
        Err(format!("Package '{}' not found", pkg_name))
    }
}

pub fn populate_json() -> io::Result<()> {
    let mut pkg_list = Vec::new();
    for entry in fs::read_dir(&*META)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            match form_package(pkg_str) {
                Ok(p) => pkg_list.push(p),
                Err(e) if e == "refused" => continue,
                Err(e) => die!("Error processing '{}': {}", pkg_str, e),
            }
        }
    }

    save_package_list(&pkg_list);
    Ok(())
}

pub fn append_json(pkg_list: &mut Vec<Package>) -> io::Result<()> {
    for entry in fs::read_dir(&*META)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            if !pkg_list.iter().any(|p| p.name == pkg_str) {
                match form_package(pkg_str) {
                    Ok(p) => pkg_list.push(p),
                    Err(e) if e == "refused" => continue,
                    Err(e) => erm!("Error processing '{}': {}", pkg_str, e),
                }
            }
        }
    }

    save_package_list(pkg_list);
    Ok(())
}
