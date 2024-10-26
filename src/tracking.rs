// tracking.rs
//
// responsible for keeping track of packages

use crate::erm;
use crate::package::{form_package, Package, PackageStatus};
use crate::paths::{META, PKGSJSON};
use serde_json::{from_str, to_string_pretty};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn load_package_list(file_path: &Path) -> io::Result<Vec<Package>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let package_list: Vec<Package> = from_str(&content).expect("Failed to parse packages.json");
    Ok(package_list)
}

pub fn save_package_list(pkg_list: &Vec<Package>, file_path: &Path) -> io::Result<()> {
    let jsdata = to_string_pretty(pkg_list).expect("Failed to serialize package data");
    let mut file = File::create(file_path)?;
    file.write_all(jsdata.as_bytes())?;
    Ok(())
}

fn build_failed() -> bool {
    Path::new("/tmp/rid/failed").exists()
}

pub fn add_package(pkg_list: &mut Vec<Package>, pkg_str: &str) -> Result<(), String> {
    if build_failed() {
        return Err("Not tracking due to build failure".to_string());
    }

    match form_package(pkg_str) {
        Ok(mut package) => {
            if let Some(existing_pkg) = pkg_list.iter_mut().find(|p| p.name == package.name) {
                existing_pkg.status = PackageStatus::Installed;
            } else {
                package.status = PackageStatus::Installed;
                pkg_list.push(package);
            }
            let _ = save_package_list(pkg_list, PKGSJSON.as_path());
            Ok(())
        }
        Err(_) => todo!(),
    }
}

pub fn remove_package(pkg_list: &mut Vec<Package>, pkg_name: &str) -> Result<(), String> {
    if let Some(package) = pkg_list.iter_mut().find(|p| p.name == pkg_name) {
        package.status = PackageStatus::Available;
        let _ = save_package_list(pkg_list, PKGSJSON.as_path());
        Ok(())
    } else {
        Err(format!("Package '{}' not found", pkg_name))
    }
}

pub fn populate_json() -> io::Result<()> {
    let mut package_list = Vec::new();
    for entry in fs::read_dir(&*META)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            match form_package(pkg_str) {
                Ok(package) => package_list.push(package),
                Err(e) if e == "refused" => continue,
                Err(e) => erm!("Error processing '{}': {}", pkg_str, e),
            }
        }
    }

    let _ = save_package_list(&package_list, PKGSJSON.as_path());
    Ok(())
}

pub fn append_json(package_list: &mut Vec<Package>) -> io::Result<()> {
    for entry in fs::read_dir(&*META)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            if !package_list.iter().any(|p| p.name == pkg_str) {
                match form_package(pkg_str) {
                    Ok(package) => package_list.push(package),
                    Err(e) if e == "refused" => continue,
                    Err(e) => erm!("Error processing '{}': {}", pkg_str, e),
                }
            }
        }
    }

    save_package_list(package_list, PKGSJSON.as_path())?;
    Ok(())
}

pub fn query_status(pkg_name: &str) -> Result<PackageStatus, String> {
    let packages = load_package_list(PKGSJSON.as_path())
        .map_err(|e| format!("Failed to load package list: {}", e))?;

    for package in packages {
        if package.name == pkg_name {
            return Ok(package.status);
        }
    }
    Err(format!("Package '{}' not found", pkg_name))
}
