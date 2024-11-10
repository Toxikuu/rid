// src/fetch.rs
//
// responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use crate::flags::FORCE;
use crate::misc::static_exec;
use crate::package::{Package, PackageStatus};
use crate::paths::BIN;
use crate::{erm, vpr};
use std::error::Error;

#[cfg(not(feature = "offline"))]
mod online {
    pub use crate::flags::DOWNLOAD;
    pub use crate::paths::{BUILDING, SOURCES};
    pub use crate::die;
    pub use std::fs::File;
    pub use ureq::get;
    pub use std::io;
}

#[cfg(not(feature = "offline"))]
use online::*;

#[cfg(not(feature = "offline"))]
fn download(p: &Package, force: bool) -> Result<(), Box<dyn Error>> {
    let url = match &p.link {
        Some(url) if !url.is_empty() => {
            vpr!("Detected url: '{}' for package {}", url, p.name);
            url
        }
        Some(_) | None => {
            vpr!("No link for {}", p.name);
            return Err("no link".into());
        }
    };

    let tb = format!("{}-{}.tar", p.name, p.version);
    let file_path = SOURCES.join(&tb);

    if file_path.exists() {
        if *DOWNLOAD.lock().unwrap() || force {
            vpr!("Forcibly downloading existing tarball for {}-{}", p.name, p.version);
        } else {
            vpr!("Skipping download for {}-{}", p.name, p.version);
            return Ok(());
        }
    } else {
        vpr!("Downloading {}", url);
    }

    if url.contains("sourceforge") {
        vpr!("Detected a sourceforge domain; I hope you have a direct url!");
    }

    let r = get(url).call()?;

    if r.status() != 200 {
        return Err(format!("Failed to download file: HTTP status {}", r.status()).into());
    }

    let mut file = File::create(&file_path)?;

    let mut reader = r.into_reader();
    io::copy(&mut reader, &mut file)?;

    Ok(())
}

// it is assumed that all offline packages must have an associated tarball
#[cfg(not(feature = "offline"))]
fn retract(p: &Package) {
    let command = format!("mkdir -pv {}/{}-{}", BUILDING.display(), p.name, p.version);
    let _ = static_exec(&command);
}

fn extract(p: &Package) -> Result<(), Box<dyn Error>> {
    if let PackageStatus::Installed = p.status {
        if !*FORCE.lock().unwrap() {
            vpr!("Not extracting tarball for installed package '{}'", p.name);
            return Ok(())
        } else {
            vpr!("Forcibly extracting tarball for installed package '{}'", p.name);
        }
    }

    let command = format!("{}/xt {} {}", BIN.display(), p.name, p.version);
    match static_exec(&command) {
        Ok(_) => Ok(()),
        Err(_) => {
            vpr!("Corrupt tarball detected!");
            Err("corrupt tarball".into())
        }
    }
}

pub fn fetch(p: &Package) {
    #[cfg(feature = "offline")]
    {
        match extract(p) {
            Ok(()) => {
                vpr!("Successfully extracted tarball");
            }
            Err(e) => erm!("Failed to extract tarball: {}", e),
        }
    }

    #[cfg(not(feature = "offline"))]
    match download(p, false) {
        Ok(_) => {
            vpr!("Successfully downloaded tarball");
            if extract(p).is_err() && { download(p, true).is_err() || extract(p).is_err() } {
                die!("Failed to recover from corrupt tarball");
            }
        }
        Err(e) if e.to_string() == "no link" => retract(p),
        Err(e) => erm!("Failed to download package: {}", e),
    }
}
