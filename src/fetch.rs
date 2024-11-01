// src/fetch.rs
//
// responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use crate::flags::FORCE;
use crate::misc::exec;
use crate::package::{Package, PackageStatus};
use crate::paths::{BUILDING, RBIN};
use crate::{erm, vpr};
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
fn download(p: &Package) -> Result<String, Box<dyn Error>> {
    let url = match &p.link {
        Some(url) => {
            vpr!("Detected url: '{}' for package {}", url, p.name);

            if !url.is_empty() {
                url
            } else {
                vpr!("Package '{}' has no link!", p.name);
                return Ok("no link".to_string()); // might replace these with Err()
            }
        }
        _ => {
            vpr!("Package '{}' has no link!", p.name);
            return Ok("no link".to_string()); // might replace these with Err()
        }
    };

    let tarball = format!("{}-{}.tar", p.name, p.version);
    let file_path = SOURCES.join(&tarball);

    if Path::new(&file_path).exists() {
        if !*DOWNLOAD.lock().unwrap() {
            vpr!("Skipping download for existing tarball '{}'", tarball);
            return Ok(tarball);
        } else {
            vpr!("Forcibly downloading existing tarball '{}'", tarball);
        }
    } else {
        vpr!("Downloading {}", url);
    }

    if url.contains("sourceforge") {
        vpr!("Detected a sourceforge domain; I hope you have a direct url!");
    }

    let r = get(url)?;

    if r.status().is_success() {
        let mut file = File::create(&file_path)?;
        let content = r.bytes()?;
        file.write_all(&content)?;

        Ok(tarball)
    } else {
        Err(format!("Failed to download tarball: HTTP status {}", r.status()).into())
    }
}

fn extract(tarball: &str, p: &Package) -> io::Result<()> {
    if tarball == "no link" {
        let command = format!("mkdir -pv {}/{}-{}", BUILDING.display(), p.name, p.version);
        let _ = exec(&command);
        return Ok(());
    }

    match p.status {
        PackageStatus::Installed => {
            if !*FORCE.lock().unwrap() {
                vpr!("Not extracting tarball for installed package '{}'", p.name);
                return Ok(());
            } else {
                vpr!(
                    "Forcibly extracting tarball for installed package '{}'",
                    p.name
                );
            }
        }
        _ => {
            // do nothing
        }
    }

    let command = format!("{}/xt {} {} {}", RBIN.display(), tarball, p.name, p.version);

    exec(&command).map_err(|e| {
        erm!("Execution failed: {}", e);
        io::Error::new(io::ErrorKind::Other, format!("Execution failed: {}", e))
    })?;

    Ok(())
}

pub fn wrap(p: &Package) {
    #[cfg(feature = "offline")]
    {
        let tarball = format!("{}-{}.tar", p.name, p.version);
        match extract(&tarball, &p) {
            Ok(()) => {
                vpr!("Successfully extracted tarball");
            }
            Err(e) => erm!("Failed to extract tarball: {}", e),
        }
    }

    #[cfg(not(feature = "offline"))]
    match download(p) {
        Ok(tarball) => {
            vpr!("Successfully downloaded tarball");
            match extract(&tarball, p) {
                Ok(()) => {
                    vpr!("Successfully extracted tarball");
                }
                Err(e) => erm!("Failed to extract tarball: {}", e),
            }
        }
        Err(e) => erm!("Failed to download package: {}", e),
    }
}
