// src/package.rs

use crate::misc::static_exec;
use crate::paths::RBIN;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub link: Option<String>,
    pub deps: Vec<String>,
}

pub fn form_package(pkg_str: &str) -> Result<Package, String> {
    if pkg_str == ".git" || pkg_str == "README.md" || pkg_str == "LICENSE" {
        return Err("refused".to_string());
    }

    let mut name = String::new();
    let mut version = String::new();
    let mut link = None;
    let mut deps = Vec::new();

    let command = format!("{}/mint v {}", RBIN.display(), pkg_str);
    match static_exec(&command) {
        Ok(output) => {
            for line in output.lines() {
                match line {
                    _ if line.starts_with("NAME: ") => name = line[6..].trim().to_string(),
                    _ if line.starts_with("VERS: ") => version = line[6..].trim().to_string(),
                    _ if line.starts_with("LINK: ") => link = Some(line[6..].trim().to_string()),
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
                return Err(format!("No name for package '{}'", pkg_str));
            }

            Ok(Package {
                name,
                version,
                link,
                deps,
            })
        }
        Err(e) => Err(format!("Failed to form package '{}': {}", pkg_str, e)),
    }
}
