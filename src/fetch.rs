// src/fetch.rs

// Responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use reqwest::blocking::get;
use std::error::Error;
use std::env::set_current_dir;
use std::io::{self, Write};
use std::fs::File;
use std::path::Path;
use crate::misc::exec;
use crate::paths::{SOURCES, UTILS};
use crate::package::Package;
use crate::flags::{FORCE, FULL_FORCE};
use crate::tracking::query_status;

use crate::pr;

fn download(url: &str) -> Result<String, Box<dyn Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let file_path = SOURCES.join(file_name);

    if Path::new(&file_path).exists() {
        if !*FORCE.lock().unwrap() {
            pr!(format!("Skipping download for extant tarball '{}'", file_name));
            return Ok(file_name.to_string())
        } else {
            pr!(format!("Forcefully redownloading extant tarball '{}'", file_name));
        }
    }

    let r = get(url)?;
    if r.status().is_success() {
        let mut file = File::create(&file_path)?;

        let content = r.bytes()?;
        file.write_all(&content)?;

        Ok(file_name.to_string())
    } else {
        Err(format!("Failed to download tarball: HTTP status {}", r.status()).into())
    }
}

fn extract(tarball: &str, pkg_str: &str, vers: &str) -> io::Result<()> {
    set_current_dir(&*SOURCES).map_err(|e| {
        eprintln!("Failed to change directory: {}", e);
        e
    })?;

    match query_status(pkg_str) {
        Ok(status) => {
            pr!(format!("Status: {}", status), 'v');

            match status {
                "installed" => {
                    if !*FULL_FORCE.lock().unwrap() {
                        pr!(format!("Not extracting tarball for installed package '{}'", pkg_str));
                        return Ok(());
                    } else {
                        pr!(format!("Forcibly extracting tarball for installed package '{}'", pkg_str));
                    }
                },
                "available" => {},
                _ => {
                    pr!(format!("Package '{}' unavailable", pkg_str));
                    return Ok(());
                }
            }

            let command = format!(
                "rm -rf /tmp/rid/extraction/* && tar xvf {}/{} -C /tmp/rid/extraction && {}/overwrite-dir.sh {}-{}", 
                SOURCES.display(), tarball, UTILS.display(), pkg_str, vers
            );

            exec(&command).map_err(|e| {
                eprintln!("Execution failed: {}", e);
                io::Error::new(io::ErrorKind::Other, format!("Execution failed: {}", e))
            })?;

            Ok(())
        },
        Err(e) => {
            eprintln!("Error querying package: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, format!("Error querying package: {}", e)))
        }
    }
}

pub fn wrap(pkg: &Package) {
    match &pkg.link {
        Some(link) => {
            pr!(format!("Downloading {}", link));
            match download(link) {
                Ok(tarball) => {
                    match extract(&tarball, &pkg.name, &pkg.version) {
                        Ok(()) => {
                            pr!("Extracted tarball", 'v')
                        },
                        Err(e) => eprintln!("Failed to extract tarball: {}", e),
                    }
                },
                Err(e) => eprintln!("Failed to download package: {}", e),
            }
        },
        _ => eprintln!("Package has no link"),
    }
}
