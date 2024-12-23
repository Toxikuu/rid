// src/package.rs
//
// defines core package-related functionality

use crate::cmd::static_exec;
use crate::paths::{BIN, REPO};
use crate::sets::handle_sets;
use crate::{die, vpr};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageStatus {
    Available,
    Installed,
    Removed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub deps: Vec<String>,
    pub description: String,
    pub downloads: Vec<String>,
    pub installed_version: String,
    pub link: String,
    pub name: String,
    pub news: String,
    pub status: PackageStatus,
    pub upstream: String,
    pub version: String,
    pub version_command: String,
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Package {}

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

        let mut deps = Vec::new();
        let mut description = String::new();
        let mut downloads = Vec::new();
        let mut link = String::new();
        let mut name = String::new();
        let mut news = String::new();
        let mut upstream = String::new();
        let mut version = String::new();
        let mut version_command = String::new();

        let command = format!(r#"RIDREPO="{}" {}/mint v {}"#, &*REPO, BIN.display(), pkg_name);

        let output = match static_exec(&command) {
            Ok(output) => output,
            Err(e) => die!("Failed to form package: {}", e)
        };

        for line in output.lines() {
            match line {
                _ if line.starts_with("DESC: ") => description = line[6..].trim().to_string(),
                _ if line.starts_with("LINK: ") => link = line[6..].trim().to_string(),
                _ if line.starts_with("NAME: ") => name = line[6..].trim().to_string(),
                _ if line.starts_with("NEWS: ") => news = line[6..].trim().to_string(),
                _ if line.starts_with("UPST: ") => upstream = line[6..].trim().to_string(),
                _ if line.starts_with("VCMD: ") => version_command = line[6..].trim().to_string(),
                _ if line.starts_with("VERS: ") => version = line[6..].trim().to_string(),
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

        if !pkglist.is_empty() {
            deps = handle_sets(deps, &pkglist);
        }

        let (status, installed_version) = pkglist
            .iter()
            .find(|p| p.name == name)
            .map_or((PackageStatus::Available, String::new()), |p| {
                (p.status.clone(), p.installed_version.clone())
            });

        Package {
            deps,
            description,
            downloads,
            installed_version,
            link,
            name,
            news,
            status,
            upstream,
            version,
            version_command,
        }
    }
}
