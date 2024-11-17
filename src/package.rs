// src/package.rs
//
// defines core package-related functionality

use crate::misc::static_exec;
use std::io::BufReader;
use std::fs::File;
use crate::{vpr, die};
use crate::paths::{PKGSJSON, BIN};
use crate::tracking::load_package_list;
use crate::sets::handle_sets;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageStatus {
    Available,
    Installed,
    Removed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub installed_version: String,
    pub link: String,
    pub upstream: String,
    pub selector: String,
    pub news: String,
    pub deps: Vec<String>,
    pub downloads: Vec<String>,
    pub status: PackageStatus,
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn defp(pkg_str: &str) -> Package {
    vpr!("Defining {} from json...", pkg_str);
    let file = File::open(&*PKGSJSON).expect("Failed to open $RIDPKGSJSON");
    let reader = BufReader::new(file);
    let stream: Vec<Package> = serde_json::from_reader(reader).expect("Error parsing $RIDPKGSJSON");

    for pkg_data in stream {
        if pkg_data.name == pkg_str {
            vpr!("Assigned package data to {}", pkg_str);
            return pkg_data
        }
    }

    die!("Package '{}' not found in $RIDPKGSJSON", pkg_str);
}

pub fn form_package(pkg_str: &str) -> Result<Package, String> {
    if  pkg_str == ".git"      ||
        pkg_str == "README.md" || 
        pkg_str == "LICENSE" 
    { return Err("refused".to_string()) }

    // will soon be deprecated
    let pkg_str = if pkg_str.contains("_") {
        pkg_str.replace("_", "-")
    } else {
        pkg_str.to_string()
    };

    vpr!("Forming {}", pkg_str);

    let mut name = String::new();
    let mut version = String::new();
    let mut link = String::new();
    let mut upstream = String::new();
    let mut selector = String::new();
    let mut news = String::new();
    let mut deps = Vec::new();
    let mut downloads = Vec::new();

    let command = format!("{}/mint v {}", BIN.display(), pkg_str);
    match static_exec(&command) {
        Ok(output) => {
            for line in output.lines() {
                match line {
                    _ if line.starts_with("NAME: ") => name = line[6..].trim().to_string(),
                    _ if line.starts_with("VERS: ") => version = line[6..].trim().to_string(),
                    _ if line.starts_with("LINK: ") => link = line[6..].trim().to_string(),
                    _ if line.starts_with("UPST: ") => upstream = line[6..].trim().to_string(),
                    _ if line.starts_with("SELE: ") => selector = line[6..].trim().to_string(),
                    _ if line.starts_with("NEWS: ") => news = line[6..].trim().to_string(),
                    _ if line.starts_with("DEPS: ") => deps = line[6..].split_whitespace().map(|s| s.to_string()).collect(),
                    _ if line.starts_with("DOWN: ") => downloads = line[6..].split_whitespace().map(|s| s.to_string()).collect(),
                    _ => (),
                }
            }

            if name.is_empty() {
                return Err("NAME variable is empty".to_string());
            }

            let deps = handle_sets(deps);

            let package_list = load_package_list();
            let (status, installed_version) = package_list
                .iter()
                .find(|p| p.name == name)
                .map_or((PackageStatus::Available, String::new()), |p| (p.status.clone(), p.installed_version.clone()));

            Ok(Package {
                name,
                version,
                installed_version,
                link,
                upstream,
                selector,
                news,
                deps,
                downloads,
                status,
            })
        }
        Err(e) => Err(format!("Failed to form package '{}': {}", pkg_str, e)),
    }
}
