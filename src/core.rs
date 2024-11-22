// core.rs
//
// defines core functionality

use std::io::{self, Write};
use std::fs::File;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::path::Path;
use ureq::{Response, get};
use crate::paths::{BUILDING, BIN, SOURCES};
use crate::package::Package;
use crate::resolve::find_dependants;
use crate::{erm, yn, vpr, die};
use crate::misc::{static_exec, exec};
use crate::utils::{display_list, mkdir};

pub fn mint(a: char, p: &Package) {
    let command = format!("{}/mint {} {}", BIN.display(), a, p.name);
    if let Err(e) = exec(&command) {
        die!("Failed to evaluate action '{}': {}", a, e)
    }
}

const BAR: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {bytes}/{total_bytes} ({eta})";

fn dl_bar(
    r: Response,
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
        ProgressStyle::with_template(BAR)
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

pub fn download(p: Package, force: bool) {
    let tarball = format!("{}.tar", p);
    let tarball_link = p.link;
    let tarball_path = &SOURCES.join(&tarball);
    let extra_links = p.downloads;

    if !tarball_path.exists() || force {
        vpr!("Downloading '{}' from '{}'...", tarball, tarball_link);
        let r = get(&tarball_link).call().expect("Failed to get tarball");

        if let Err(e) = dl_bar(r, &tarball, tarball_path) {
            die!("Failed to download url '{}': {}", tarball_link, e)
        }
    }

    for url in extra_links {
        let file_name = url.split('/').last().expect("Invalid url");
        let file_path = &SOURCES.join(file_name);

        if !file_path.exists() || force {
            vpr!("Downloading '{}' from '{}'...", file_name, url);
            let r = get(&url).call().expect("Failed to get url");

            if let Err(e) = dl_bar(r, file_name, file_path) {
                die!("Failed to download url '{}': {}", url, e)
            }
        }
    }
}

// NOTE: whether a package should be extracted will now be handled under pm.install()
pub fn extract(p: &Package) -> Result<(), Box<dyn Error>> {
    if p.link.is_empty() {
        let path = &BUILDING.join(p.to_string());
        mkdir(path);
        return Ok(())
    }

    let command = format!("{}/xt {}", BIN.display(), p);
    if static_exec(&command).is_err() {
        vpr!("Corrupt tarball detected!");
        return Err("corrupt tarball".into())
    }
    
    Ok(())
}

pub fn fetch(p: &Package) {
    download(p.clone(), false);
    if extract(p).is_err() {
        download(p.clone(), true);
        if extract(p).is_err() {
            die!("Failed to recover from corrupt tarball")
        }
    }
}

pub fn confirm_removal(pkg: &Package, pkglist: &[Package]) -> bool {
    vpr!("Checking dependants for '{}'", pkg);
    let dependants = find_dependants(pkg.clone(), pkglist.to_vec());
    let len = dependants.len();

    vpr!("Found {} dependants", len);
    if dependants.is_empty() { return true }

    erm!("Found {} dependant packages:", len);
    display_list(dependants);

    let message = format!("Remove '{}' ({} dependants)?", pkg, len);
    yn!(&message, false)
}
