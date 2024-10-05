// src/fetch.rs

// Responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use std::error::Error;
use std::env::set_current_dir;
use std::io;
use crate::misc::exec;
use crate::paths::{SOURCES, UTILS};
use crate::package::Package;

fn download(url: &str) -> Result<String, Box<dyn Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let command = format!("wget -q --show-progress {} -O '{}/{}'", url, SOURCES.display(), file_name);

    match exec(&command) {
        Ok(_) => Ok(file_name.to_string()), // returns tarball
        Err(e) => Err(format!("Failed to download tarball: {}", e).into()),
    }
}

fn extract(tarball: &str, pkg_str: &str, vers: &str) -> io::Result<()> {
    set_current_dir(&*SOURCES).map_err(|e| {
        eprintln!("Failed to change directory: {}", e);
        e
    })?;

    //let command = format!("tar xvf {} -C /tmp/rid/extraction && mv -Tfv /tmp/rid/extraction/* /tmp/rid/building/{}-{}", tarball, pkg_str, vers);
    let command = format!("tar xvf {} -C /tmp/rid/extraction && {}/overwrite-dir.sh {}-{}", tarball, UTILS.display(), pkg_str, vers);

    match exec(&command) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }

    Ok(())
}

pub fn wrap(pkg: Package) {
    match &pkg.link {
        Some(link) => {
            println!("Downloading {}", link);
            match download(link) {
                Ok(tarball) => {
                    match extract(&tarball, &pkg.name, &pkg.version) {
                        Ok(()) => {
                            println!("Extracted tarball")
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
