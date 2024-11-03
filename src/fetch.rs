// src/fetch.rs
//
// responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use crate::flags::FORCE;
use crate::misc::exec;
use crate::package::{Package, PackageStatus};
use crate::paths::RBIN;
use crate::{erm, vpr};
use std::io;

#[cfg(not(feature = "offline"))]
mod online {
    pub use crate::flags::DOWNLOAD;
    pub use crate::paths::{BUILDING, SOURCES};
    pub use glob::glob;
    pub use std::error::Error;
    pub use std::fs::File;
    pub use std::io::Write;
    pub use std::path::Path;
    pub use ureq::get;
}

#[cfg(not(feature = "offline"))]
use online::*;

#[cfg(not(feature = "offline"))]
fn download(p: &Package) -> Result<(), Box<dyn Error>> {
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

    let tarball_pattern = format!("{}-{}.t*", p.name, p.version);
    let file_pattern = SOURCES.join(&tarball_pattern);

    let file_exists = glob(file_pattern.to_str().unwrap())
        .expect("Failed to read glob pattern")
        .any(|entry| entry.is_ok() && Path::new(&entry.unwrap()).exists());

    if file_exists {
        if !*DOWNLOAD.lock().unwrap() {
            vpr!("Skipping download for {}-{}", p.name, p.version);
            return Ok(());
        } else {
            vpr!(
                "Forcibly downloading existing tarball for {}-{}",
                p.name,
                p.version
            );
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

    let tb = format!("{}-{}.tar", p.name, p.version);
    let file_path = SOURCES.join(&tb);

    let mut file = File::create(&file_path)?;

    match r.header("Content-Type") {
        Some(content_type) if content_type.starts_with("text/") => {
            let text = r.into_string()?;
            file.write_all(text.as_bytes())?;
        }
        _ => {
            let mut reader = r.into_reader();
            io::copy(&mut reader, &mut file)?;
        }
    }

    Ok(())
}

// it is assumed that all offline packages must have an associated tarball
#[cfg(not(feature = "offline"))]
fn retract(p: &Package) {
    let command = format!("mkdir -pv {}/{}-{}", BUILDING.display(), p.name, p.version);
    let _ = exec(&command);
}

fn extract(p: &Package) -> io::Result<()> {
    if let PackageStatus::Installed = p.status {
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

    let command = format!("{}/xt {} {}", RBIN.display(), p.name, p.version);

    exec(&command).map_err(|e| {
        erm!("Execution failed: {}", e);
        io::Error::new(io::ErrorKind::Other, format!("Execution failed: {}", e))
    })?;

    Ok(())
}

pub fn wrap(p: &Package) {
    #[cfg(feature = "offline")]
    {
        match extract(&p) {
            Ok(()) => {
                vpr!("Successfully extracted tarball");
            }
            Err(e) => erm!("Failed to extract tarball: {}", e),
        }
    }

    #[cfg(not(feature = "offline"))]
    match download(p) {
        Ok(()) => {
            vpr!("Successfully downloaded tarball");
            match extract(p) {
                Ok(()) => {
                    vpr!("Extracted tarball for '{}'", p.name);
                }
                Err(e) => erm!("Failed to extract tarball for '{}': {}", p.name, e),
            }
        }
        Err(e) if e.to_string() == "no link" => retract(p),
        Err(e) => erm!("Failed to download package: {}", e),
    }
}
