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
    pub link: Option<String>,
    pub upstream: Option<String>,
    pub selector: Option<String>,
    pub deps: Vec<String>,
    pub status: PackageStatus,
}

pub fn defp(pkg: &str) -> Package {
    vpr!("Defining {} from json", pkg);
    let file = File::open(&*PKGSJSON).expect("Failed to open $RIDPKGSJSON");
    let reader = BufReader::new(file);

    let stream: Vec<Package> = serde_json::from_reader(reader).expect("Error parsing $RIDPKGSJSON");

    for pkg_data in stream {
        if pkg_data.name == pkg {
            return pkg_data
        }
    }

    die!("Package '{}' not found in $RIDPKGSJSON", pkg);
}

pub fn form_package(pkg_str: &str) -> Result<Package, String> {
    if  pkg_str == ".git"      ||
        pkg_str == "README.md" || 
        pkg_str == "LICENSE" 
    { return Err("refused".to_string()) }

    let pkg_str = if pkg_str.contains("-") {
        pkg_str.replace("-", "_")
    } else {
        pkg_str.to_string()
    };

    vpr!("Forming {}", pkg_str);

    let mut name = String::new();
    let mut version = String::new();
    let mut link = None;
    let mut upstream = None;
    let mut selector = None;
    let mut deps = Vec::new();

    let command = format!("{}/mint v {}", BIN.display(), pkg_str);
    match static_exec(&command) {
        Ok(output) => {
            for line in output.lines() {
                match line {
                    _ if line.starts_with("NAME: ") => name = line[6..].trim().to_string(),
                    _ if line.starts_with("VERS: ") => version = line[6..].trim().to_string(),
                    _ if line.starts_with("LINK: ") => link = Some(line[6..].trim().to_string()),
                    _ if line.starts_with("UPST: ") => upstream = Some(line[6..].trim().to_string()),
                    _ if line.starts_with("SELE: ") => selector = Some(line[6..].trim().to_string()),
                    _ if line.starts_with("DEPS: ") => deps = line[6..].split_whitespace().map(|s| s.to_string()).collect(),
                    _ => (),
                }
            }

            if name.is_empty() {
                return Err("NAME variable is empty".to_string());
            }

            let deps = handle_sets(deps);

            let package_list = load_package_list();
            let status = package_list
                .iter()
                .find(|p| p.name == name)
                .map_or(PackageStatus::Available, |p| p.status.clone());

            Ok(Package {
                name,
                version,
                link,
                upstream,
                selector,
                deps,
                status,
            })
        }
        Err(e) => Err(format!("Failed to form package '{}': {}", pkg_str, e)),
    }
}
