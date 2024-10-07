// src/package.rs

use crate::paths::RBIN;
use crate::misc::static_exec;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub link: Option<String>,
    pub deps: Vec<String>,
}

impl Package {
    //pub fn format(&self, max_name_length: usize, status: &str) -> String {
    //    format!("{:<max_name_length$} ~ {}", format!("{}-{}", self.name, self.version), status)
    //}
}

pub fn form_package(pkg_str: &str) -> Result<Package, String> {
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
                    },
                    _ => (),
                }
            }

            Ok(Package { name, version, link, deps })
        },
        Err(e) => Err(format!("Failed to form package '{}': {}", pkg_str, e)),
    }
}
