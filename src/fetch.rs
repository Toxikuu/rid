// src/fetch.rs
//
// responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use crate::flags::FORCE;
use crate::misc::exec;
use crate::package::{Package, PackageStatus};
use crate::tracking::query_status;
use crate::{erm, pr};
use std::io;

#[cfg(not(feature = "offline"))]
mod online {
    pub use crate::flags::DOWNLOAD;
    pub use crate::paths::SOURCES;
    pub use reqwest::blocking::get;
    pub use std::error::Error;
    pub use std::fs::File;
    pub use std::io::Write;
    pub use std::path::Path;
}

#[cfg(not(feature = "offline"))]
use online::*;

#[cfg(not(feature = "offline"))]
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
                erm!("Package '{}' has no link!", pkg.name);
                return Ok("no link".to_string());
            }
        }
        _ => {
            erm!("Package '{}' has no link!", pkg.name);
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
        pr!(
            "Detected a sourceforge domain; I hope you have a direct url!",
            'v'
        )
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
                        pr!(
                            format!(
                                "Forcibly extracting tarball for installed package '{}'",
                                pkg_str
                            ),
                            'v'
                        );
                    }
                }
                PackageStatus::Available => {}
                _ => {
                    pr!(format!("Package '{}' unavailable", pkg_str));
                    return Ok(());
                }
            }

            let command = format!("/etc/rid/rbin/xt {} {} {}", tarball, pkg_str, vers);

            exec(&command).map_err(|e| {
                erm!("Execution failed: {}", e);
                io::Error::new(io::ErrorKind::Other, format!("Execution failed: {}", e))
            })?;

            Ok(())
        }
        Err(e) => {
            erm!("Error querying package: {}", e);
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error querying package: {}", e),
            ))
        }
    }
}

pub fn wrap(pkg: &Package) {
    #[cfg(feature = "offline")]
    {
        let tarball = format!("{}-{}.tar", pkg.name, pkg.version);
        match extract(&tarball, &pkg.name, &pkg.version) {
            Ok(()) => {
                pr!("Successfully extracted tarball", 'v');
            }
            Err(e) => erm!("Failed to extract tarball: {}", e),
        }
    }

    #[cfg(not(feature = "offline"))]
    match download(pkg) {
        Ok(tarball) => {
            pr!("Successfully downloaded tarball", 'v');
            match extract(&tarball, &pkg.name, &pkg.version) {
                Ok(()) => {
                    pr!("Successfully extracted tarball", 'v');
                }
                Err(e) => erm!("Failed to extract tarball: {}", e),
            }
        }
        Err(e) => erm!("Failed to download package: {}", e),
    }
}
