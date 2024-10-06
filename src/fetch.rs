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
use crate::flags::FORCE;

use crate::pr;

fn download(url: &str) -> Result<String, Box<dyn Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let file_path = SOURCES.join(file_name);

    if Path::new(&file_path).exists(){
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

    let command = format!("tar xvf {} -C /tmp/rid/extraction && {}/overwrite-dir.sh {}-{}", tarball, UTILS.display(), pkg_str, vers);

    if let Err(e) = exec(&command) {
        eprintln!("Execution failed: {}", e);
    }

    Ok(())
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
