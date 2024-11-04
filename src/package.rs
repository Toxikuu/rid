// src/package.rs
//
// defines core package-related functionality

use crate::misc::static_exec;
use crate::paths::{PKGSJSON, RBIN};
use crate::tracking::load_package_list;
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

pub fn form_package(pkg_str: &str) -> Result<Package, String> {
    if pkg_str == ".git" || pkg_str == "README.md" || pkg_str == "LICENSE" {
        return Err("refused".to_string());
    }

    let pkg_str = if pkg_str.contains("-") {
        pkg_str.replace("-", "_")
    } else {
        pkg_str.to_string()
    };

    let mut name = String::new();
    let mut version = String::new();
    let mut link = None;
    let mut upstream = None;
    let mut selector = None;
    let mut deps = Vec::new();

    let command = format!("{}/mint v {}", RBIN.display(), pkg_str);
    match static_exec(&command) {
        Ok(output) => {
            for line in output.lines() {
                match line {
                    _ if line.starts_with("NAME: ") => name = line[6..].trim().to_string(),
                    _ if line.starts_with("VERS: ") => version = line[6..].trim().to_string(),
                    _ if line.starts_with("LINK: ") => link = Some(line[6..].trim().to_string()),
                    _ if line.starts_with("UPST: ") => upstream = Some(line[6..].trim().to_string()),
                    _ if line.starts_with("SELE: ") => selector = Some(line[6..].trim().to_string()),
                    _ if line.starts_with("DEPS: ") => {
                        deps = line[6..]
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect();
                    }
                    _ => (),
                }
            }

            if name.is_empty() {
                return Err("Likely nonexistent metafile".to_string());
            }

            let package_list = load_package_list(&PKGSJSON).unwrap_or_else(|_| Vec::new());
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
