// src/package.rs

use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

use crate::paths::{PKGSTXT, UTILS};
use crate::misc::exec;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub status: String,
    pub link: Option<String>,
    pub deps: Option<Vec<String>>,
}

impl Package {
    pub fn format(&self, max_name_length: usize) -> String {
        format!("{:<max_name_length$} ~ {}", format!("{}-{}", self.name, self.version), self.status)
    }
}

pub fn form_package(pkg_str: &str) -> Result<Package, String> {
    let mut name = String::new();
    let mut version = String::new();
    let mut status = String::new();
    let mut link = None;
    let mut deps = None;

    let command = format!("{}/meta-interface.sh v {}", UTILS.to_str().expect("Invalid UTF-8"), pkg_str);
    match exec(&command) {
        Ok(output) => {
            for line in output.lines() {
                match line {
                    _ if line.starts_with("NAME: ") => name = line[6..].trim().to_string(),
                    _ if line.starts_with("VERS: ") => version = line[6..].trim().to_string(),
                    _ if line.starts_with("STAT: ") => status = line[6..].trim().to_string(),
                    _ if line.starts_with("LINK: ") => link = Some(line[6..].trim().to_string()),
                    _ if line.starts_with("DEPS: ") => {
                        deps = Some(line[6..]
                            .trim()
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect());
                    },
                    _ => (),
                }
            }

            Ok(Package { name, version, status, link, deps })
        },
        Err(e) => Err(format!("Failed to form package '{}': {}", pkg_str, e)),
    }
}

pub fn read_packages_file() -> io::Result<Vec<Package>> {
    let path = Path::new(&*PKGSTXT);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut packages = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("//") || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            println!("Malformed line!");
            continue;
        }

        let name_version = parts[0];
        let status = parts[2..].join(" ");

        let parts: Vec<&str> = name_version.split('-').collect();
        if parts.len() != 2 {
            println!("Malformed name_version!");
            continue;
        }

        let link = None;
        let deps = None;

        let name = parts[0].to_string();
        let version = parts[1].to_string();
        packages.push(Package { name, version, status, link, deps })
    }

    Ok(packages)
}

pub fn track_package(new_package: Package) -> io::Result<()> {
    let path = Path::new(&*PKGSTXT);
    let file = File::options().append(true).open(path)?;
    let mut writer = BufWriter::new(file);

    let packages = read_packages_file()?;
    let max_name_length = packages.iter()
        .map(|p| format!("{}-{}", p.name, p.version).len())
        .max()
        .unwrap_or(0);

    writeln!(writer, "{}", new_package.format(max_name_length))?;

    Ok(())
}
