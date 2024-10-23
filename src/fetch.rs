// src/fetch.rs

// Responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use crate::flags::{DOWNLOAD, FORCE};
use crate::misc::exec;
use crate::package::{Package, PackageStatus};
use crate::paths::SOURCES;
use crate::tracking::query_status;
use reqwest::blocking::get;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::pr;

fn download(pkg: &Package) -> Result<String, Box<dyn Error>> {
    let url = match &pkg.link {
        Some(url) => {
            pr!(
                format!("Detected url: '{:?}' for package '{:?}'", url, pkg.name),
                'v'
            );
            if !url.is_empty() {
                url
            } else {
                eprintln!("Package '{}' has no link!", pkg.name);
                return Ok("no link".to_string());
            }
        }
        _ => {
            eprintln!("Package '{}' has no link!", pkg.name);
            return Ok("no link".to_string());
        }
    };

    let file_name = format!("{}-{}.tar", pkg.name, pkg.version);
    let file_path = SOURCES.join(&file_name);

    if Path::new(&file_path).exists() {
        if !*DOWNLOAD.lock().unwrap() {
            pr!(format!(
                "Skipping download for existing tarball '{}'",
                file_name
            ));
            return Ok(file_name);
        } else {
            pr!(format!(
                "Forcefully redownloading existing tarball '{}'",
                file_name
            ));
        }
    } else {
        pr!(format!("Downloading {}", url));
    }

    if url.contains("sourceforge") {
        pr!("Downloading sourceforge tarball with wget");
        let command = format!("wget {} -O {}", url, &file_path.to_string_lossy());
        exec(&command)?;
        return Ok(file_name);
    }

    let r = get(url)?;

    if r.status().is_success() {
        let mut file = File::create(&file_path)?;
        let content = r.bytes()?;
        file.write_all(&content)?;

        Ok(file_name)
    } else {
        Err(format!("Failed to download tarball: HTTP status {}", r.status()).into())
    }
}

fn extract(tarball: &str, pkg_str: &str, vers: &str) -> io::Result<()> {
    if tarball == "no link" {
        let command = format!("mkdir -pv /tmp/rid/building/{}-{}", pkg_str, vers);
        let _ = exec(&command);
        return Ok(());
    }

    match query_status(pkg_str) {
        Ok(status) => {
            pr!(format!("Status: {:?}", status), 'v');

            match status {
                PackageStatus::Installed => {
                    if !*FORCE.lock().unwrap() {
                        pr!(format!(
                            "Not extracting tarball for installed package '{}'",
                            pkg_str
                        ));
                        return Ok(());
                    } else {
                        pr!(format!(
                            "Forcibly extracting tarball for installed package '{}'",
                            pkg_str
                        ));
                    }
                }
                PackageStatus::Available => {}
                _ => {
                    pr!(format!("Package '{}' unavailable", pkg_str));
                    return Ok(());
                }
            }

            let command = format!(
                "rm -rf /tmp/rid/building/* && rm -rf /tmp/rid/extraction/* && tar xvf {}/{} -C /tmp/rid/extraction && mv -Tvf /tmp/rid/extraction/* /tmp/rid/building/{}-{}", 
                SOURCES.display(), tarball, pkg_str, vers
            );

            exec(&command).map_err(|e| {
                eprintln!("Execution failed: {}", e);
                io::Error::new(io::ErrorKind::Other, format!("Execution failed: {}", e))
            })?;

            Ok(())
        }
        Err(e) => {
            eprintln!("Error querying package: {}", e);
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error querying package: {}", e),
            ))
        }
    }
}

pub fn wrap(pkg: &Package) {
    match download(pkg) {
        Ok(tarball) => {
            pr!("Successfully downloaded tarball", 'v');
            match extract(&tarball, &pkg.name, &pkg.version) {
                Ok(()) => {
                    pr!("Successfully extracted tarball", 'v');
                }
                Err(e) => eprintln!("Failed to extract tarball: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to download package: {}", e),
    }
}
