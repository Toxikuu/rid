// src/fetch.rs
//
// responsible for fetching the tarball, extracting it, and entering the directory, as well as
// keeping the tarball around after.

use crate::flags::FORCE;
use crate::misc::static_exec;
use crate::package::{Package, PackageStatus};
use crate::paths::BUILDING;
use crate::paths::{BIN, SOURCES};
use crate::{die, erm, vpr};
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

const DOWNLOAD_TEMPLATE: &str =
    "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {bytes}/{total_bytes} ({eta})";

fn handle_bar(
    r: ureq::Response,
    file_name: &str,
    file_path: &Path,
) -> Result<ProgressBar, Box<dyn Error>> {
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

    bar.set_style(
        ProgressStyle::with_template(DOWNLOAD_TEMPLATE)
            .unwrap()
            .progress_chars("#|-"),
    );

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

pub fn down(url: &str, force: bool) -> Result<(), Box<dyn Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let file_path = &SOURCES.join(file_name);

    if file_path.exists() && !force {
        vpr!("Not downloading existing file: {}", file_name);
        return Err("extant".into());
    }

    vpr!("Downloading '{}' from '{}'...", file_name, url);
    let r = ureq::get(url).call()?;

    if let Err(e) = handle_bar(r, file_name, file_path) {
        die!("Failed to download url '{}': {}", url, e)
    }

    Ok(())
}

pub fn download(p: &Package, force: bool) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}-{}.tar", p.name, p.version);
    let file_path = &SOURCES.join(&file_name);

    if file_path.exists() && !force {
        vpr!("Not downloading existing file: {}", file_name);
        return Err("extant".into());
    }

    let url = &p.link;
    if !url.is_empty() {
        vpr!("Detected url: '{}' for package '{}'", url, p.name);
    } else {
        vpr!("No link for '{}'", p.name);
        return Err("no link".into());
    }

    vpr!("Downloading tarball: {}", file_name);
    let r = ureq::get(url).call()?;

    if let Err(e) = handle_bar(r, &file_name, file_path) {
        die!("Failed to download url '{}': {}", &url, e)
    }

    Ok(())
}

// it is assumed that all offline packages must have an associated tarball
fn retract(p: &Package) {
    let command = format!("mkdir -pv {}/{}-{}", BUILDING.display(), p.name, p.version);
    static_exec(&command).unwrap(); // should:tm: never fail
}

fn extract(p: &Package) -> Result<(), Box<dyn Error>> {
    if !p.link.is_empty() {
        let tarball = format!("{}/{}-{}.tar", SOURCES.display(), p.name, p.version);
        let tarball_path = Path::new(&tarball);
        if !tarball_path.exists() {
            die!("Nonexistent tarball for package '{}-{}'", p.name, p.version);
        }
    }

    if let PackageStatus::Installed = p.status {
        if !*FORCE.lock().unwrap() && p.version == p.installed_version {
            vpr!("Not extracting tarball for installed package '{}'", p.name);
            return Ok(());
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
    match download(p, false) {
        Ok(_) => {
            vpr!("Successfully downloaded tarball");
            if extract(p).is_err() && { download(p, true).is_err() || extract(p).is_err() } {
                die!("Failed to recover from corrupt tarball");
            }
        }
        Err(e) => match &*e.to_string() {
            "extant" => {
                vpr!("Tarball already exists");
                if extract(p).is_err() && { download(p, true).is_err() || extract(p).is_err() } {
                    die!("Failed to recover from corrupt tarball");
                }
            }
            "no link" => retract(p),
            _ => erm!("Failed to download package: {}", e),
        },
    }

    let dls = p.downloads.clone();
    for dl in dls {
        if let Err(e) = down(&dl, false) {
            if e.to_string() == "extant" {
                continue;
            }
            die!("Failed to download extra url '{}': {}", dl, e)
        }
    }
}
