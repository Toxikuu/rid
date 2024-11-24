// src/package.rs
//
// defines core package-related functionality

use crate::cmd::static_exec;
use crate::paths::BIN;
use crate::sets::handle_sets;
use crate::{die, vpr};
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.version)
    }
}

impl Package {
    pub fn new(name: &str, pkglist: Vec<Package>) -> Package {
        vpr!("Creating package '{}' from json...", name);
        for p in pkglist.iter() {
            if p.name == name {
                vpr!("Assigned package data to '{}'", name);
                return p.clone()
            }
        }

        Package::def(name, pkglist)
    }

    pub fn def(pkg_name: &str, pkglist: Vec<Package>) -> Package {
        vpr!("Forming {}", pkg_name);

        let mut name = String::new();
        let mut version = String::new();
        let mut link = String::new();
        let mut upstream = String::new();
        let mut selector = String::new();
        let mut news = String::new();
        let mut deps = Vec::new();
        let mut downloads = Vec::new();

        let command = format!("{}/mint v {}", BIN.display(), pkg_name);

        let output = match static_exec(&command) {
            Ok(output) => output,
            Err(e) => die!("Failed to form package: {}", e)
        };

        for line in output.lines() {
            match line {
                _ if line.starts_with("NAME: ") => name = line[6..].trim().to_string(),
                _ if line.starts_with("VERS: ") => version = line[6..].trim().to_string(),
                _ if line.starts_with("LINK: ") => link = line[6..].trim().to_string(),
                _ if line.starts_with("UPST: ") => upstream = line[6..].trim().to_string(),
                _ if line.starts_with("SELE: ") => selector = line[6..].trim().to_string(),
                _ if line.starts_with("NEWS: ") => news = line[6..].trim().to_string(),
                _ if line.starts_with("DEPS: ") => {
                    deps = line[6..]
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect()
                }
                _ if line.starts_with("DOWN: ") => {
                    downloads = line[6..]
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect()
                }
                _ => (),
            }
        }

        if name.is_empty() { die!("Missing name for package: {}", pkg_name) }
        let deps = handle_sets(deps, &pkglist);

        let (status, installed_version) = pkglist
            .iter()
            .find(|p| p.name == name)
            .map_or((PackageStatus::Available, String::new()), |p| {
                (p.status.clone(), p.installed_version.clone())
            });

        Package {
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
        }
    }
}
