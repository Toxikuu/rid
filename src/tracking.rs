// tracking.rs
//
// responsible for keeping track of packages

use crate::{die, vpr};
use crate::package::{form_package, Package, PackageStatus};
use crate::paths::{META, PKGSJSON, FAILED};
use crate::misc::get_mod_time;
use serde_json::{from_str, to_string_pretty};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

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
    file.write_all(jsdata.as_bytes()).expect("Failed to write to $RIDPKGSJSON");
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

pub fn populate_json() -> io::Result<usize> {
    // only intended to be used when $RIDPKGSJSON is empty
    let mut pkg_list = Vec::new();
    for entry in fs::read_dir(&*META)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            vpr!("Caching package '{}'...", pkg_str);

            match form_package(pkg_str) {
                Ok(p) => {
                    pkg_list.push(p);
                    save_package_list(&pkg_list);
                },
                Err(e) if e == "refused" => continue,
                Err(e) => die!("Error processing '{}': {}", pkg_str, e),
            }
        }
    }

    Ok(pkg_list.len())
}


pub fn cache_changes(pkg_list: &mut Vec<Package>) -> io::Result<u16> {
    // caches changes made in $RIDMETA to $RIDPKGSJSON
    let json_mod_time = get_mod_time(&PKGSJSON)?;
    let mut updated_count: u16 = 0;
    
    for entry in fs::read_dir(&*META)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
            let entry_mod_time = get_mod_time(&path)?;

            if entry_mod_time >= json_mod_time {
                vpr!("Caching package '{}'...", pkg_str);

                match form_package(pkg_str) {
                    Ok(pkg) => {
                        if let Some(pos) = pkg_list.iter_mut().position(|p| p.name == pkg.name) {
                            pkg_list[pos] = pkg;
                            updated_count += 1;
                        } else {
                            pkg_list.push(pkg);
                            updated_count += 1;
                        }

                        if updated_count > 0 {
                            save_package_list(pkg_list);
                        }
                    },
                    Err(e) if e == "refused" => continue,
                    Err(e) => die!("Error processing '{}': {}", pkg_str, e)
                }
            }
        }
    }

    Ok(updated_count)
}

// pub fn append_json(pkg_list: &mut Vec<Package>) -> io::Result<()> {
//     for entry in fs::read_dir(&*META)? {
//         let entry = entry?;
//         let path = entry.path();
//
//         if let Some(pkg_str) = path.file_name().and_then(|n| n.to_str()) {
//             if !pkg_list.iter().any(|p| p.name == pkg_str) {
//                 vpr!("Appending package '{}' to pkgs.json", pkg_str);
//                 match form_package(pkg_str) {
//                     Ok(p) => pkg_list.push(p),
//                     Err(e) if e == "refused" => continue,
//                     Err(e) => erm!("Error processing '{}': {}", pkg_str, e),
//                 }
//             }
//         }
//     }
//
//     save_package_list(pkg_list);
//     Ok(())
// }
