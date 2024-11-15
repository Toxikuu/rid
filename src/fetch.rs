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
    pub use std::path::Path;
    pub use std::io::{self, Write};
    pub use indicatif::{ProgressBar, ProgressStyle};
}

#[cfg(not(feature = "offline"))]
use online::*;

#[cfg(not(feature = "offline"))]
const DOWNLOAD_TEMPLATE: &str =
    "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {bytes}/{total_bytes} ({eta})";

#[cfg(not(feature = "offline"))]
fn handle_bar(r: ureq::Response, file_name: &str, file_path: &Path) -> Result<ProgressBar, Box<dyn Error>> {
    if r.status() != 200 {
        return Err(format!("Non-200 HTTP status: {}", r.status()).into());
    }

    let length = r.header("Content-Length").and_then(|len| len.parse().ok());
    let bar = match length {
        Some(len) => ProgressBar::new(len),
        _ => ProgressBar::new_spinner(),
    };

    let message = format!("Downloading {}", file_name);
    bar.set_message(message);

    bar.set_style(ProgressStyle::with_template(DOWNLOAD_TEMPLATE).unwrap().progress_chars("#|-"));

    if let Some(len) = length {
        bar.set_length(len);
    }

    let mut f = File::create(file_path)?;
    match r.header("Content-Type") {
        Some(ct) if ct.starts_with("text/") => {
            let text = r.into_string()?;
            f.write_all(text.as_bytes())?;
        }
        _ => {
            io::copy(&mut bar.wrap_read(r.into_reader()), &mut f).map(|_| ())?;
        }
    }
    bar.finish_with_message("Download complete");
    Ok(bar)
} 

#[cfg(not(feature = "offline"))]
fn down(url: &str) -> Result<(), Box<dyn Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let file_path = &SOURCES.join(file_name);

    if file_path.exists() && !*DOWNLOAD.lock().unwrap() {
        vpr!("Not downloading existing file: {}", file_name);
        return Ok(());
    }

    vpr!("Forcibly downloading existing file: {}", file_name);
    let r = ureq::get(url).call()?;

    if let Err(e) = handle_bar(r, file_name, file_path) {
        die!("Failed to download url '{}': {}", url, e)
    }

    Ok(())
}

#[cfg(not(feature = "offline"))]
fn download(p: &Package, force: bool) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}-{}.tar", p.name, p.version);
    let file_path = &SOURCES.join(&file_name);

    if file_path.exists() && !force {
        vpr!("Not downloading existing file: {}", file_name);
        return Ok(());
    }

    let url = &p.link;
    if !url.is_empty() {
        vpr!("Detected url: '{}' for package '{}'", url, p.name);
    } else {
        vpr!("No link for '{}'", p.name);
        return Err("no link".into())
    }

    vpr!("Forcibly downloading existing file: {}", file_name);
    let r = ureq::get(url).call()?;

    if let Err(e) = handle_bar(r, &file_name, file_path) {
        die!("Failed to download url '{}': {}", &url, e)
    }

    Ok(())
}

// it is assumed that all offline packages must have an associated tarball
#[cfg(not(feature = "offline"))]
fn retract(p: &Package) {
    let command = format!("mkdir -pv {}/{}-{}", BUILDING.display(), p.name, p.version);
    let _ = static_exec(&command); // TODO: Error handle all 'let _ =' better
}

fn extract(p: &Package) -> Result<(), Box<dyn Error>> {
    if let PackageStatus::Installed = p.status {
        if !*FORCE.lock().unwrap() && p.version == p.installed_version {
            vpr!("Not extracting tarball for installed package '{}'", p.name);
            return Ok(())
        } else {
            vpr!("Extracting tarball for installed package '{}'", p.name);
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
    {
        let force = *DOWNLOAD.lock().unwrap();
        match download(p, force) {
            Ok(_) => {
                vpr!("Successfully downloaded tarball");
                if extract(p).is_err() && { download(p, true).is_err() || extract(p).is_err() } {
                    die!("Failed to recover from corrupt tarball");
                }
            }
            Err(e) if e.to_string() == "no link" => retract(p),
            Err(e) => erm!("Failed to download package: {}", e),
        }
       
        let dls = p.downloads.clone();
        if !dls.is_empty() {
            for dl in dls {
                if let Err(e) = down(&dl) {
                    die!("Failed to download extra url '{}': {}", dl, e)
                }
            }
        }
    }
}
