// src/bootstrap.rs
//
// responsible for bootsrapping rid

use crate::misc::exec;
use crate::paths::*;
use crate::tracking::create_json;
use crate::{die, vpr};
use crate::{erm, msg};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;
use ureq::get;

fn dl(url: &str, outdir: &Path) -> Result<(), Box<dyn Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let file_path = outdir.join(file_name);

    vpr!("Downloading {} to {}", url, outdir.display());
    let r = get(url).call()?;

    if r.status() != 200 {
        return Err(format!("Failed to download file: HTTP status {}", r.status()).into());
    }

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

fn get_rid() {
    let link = format!(
        "https://github.com/Toxikuu/rid/releases/download/v{}/rid-root.tar.xz",
        env!("CARGO_PKG_VERSION")
    );

    match dl(&link, &BUILDING) {
        Ok(_) => vpr!("Downloaded rid-root tarball to {}", BUILDING.display()),
        Err(e) => {
            erm!("Failed to download rid-root tarball: {}", e);
            exit(1);
        }
    }

    let command = format!(
        "cd {} && tar xf rid-root.tar.xz -C {}",
        BUILDING.display(),
        RIDHOME.display()
    );

    match exec(&command) {
        Ok(_) => vpr!("Extracted rid-root"),
        Err(e) => {
            erm!("Failed to set up rid: {}", e);
            exit(1);
        }
    }
}

pub fn get_rid_meta(overwrite: bool) {
    // used for bootstrapping and syncing
    match dl(
        "https://github.com/Toxikuu/rid-meta/archive/refs/heads/master.tar.gz",
        &BUILDING,
    ) {
        Ok(_) => vpr!("Downloaded rid-meta tarball"),
        Err(e) => {
            erm!("Failed to download rid-meta tarball: {}", e);
            exit(1);
        }
    }

    let c = if overwrite { ' ' } else { 'n' };
    let command = format!(
        "cd     {}                                    && \
        tar xvf master.tar.gz                         && \
        rm -vf  rid-meta-master/{{LICENSE,README.md}} && \
        mv -v{} rid-meta-master/* {}                  && \
        rm -rvf master.tar.gz rid-meta-master",
        BUILDING.display(),
        c,
        META.display(),
    );

    match exec(&command) {
        Ok(_) => msg!("Synced!"),
        Err(e) => {
            erm!("Failed to sync rid-meta: {}", e);
            exit(1)
        }
    }
}

fn bootstrap() {
    get_rid();
    get_rid_meta(false);

    let command = format!(
        "touch {}            &&
                chmod 666 {} &&
                chmod 755 {}/*",
        PKGSJSON.display(),
        PKGSJSON.display(),
        BIN.display(),
    );
    match exec(&command) {
        Ok(_) => vpr!("Made files in bin executable"),
        Err(e) => {
            erm!("Failed to make files in bin executable: {}", e);
            exit(1);
        }
    }

    // cleanup
    let command = format!(
        "cd {} && rm -rf .git* Cargo.* src TDL LICENSE README.md",
        RIDHOME.display()
    );
    match exec(&command) {
        Ok(_) => vpr!("Cleaned extras from {}", RIDHOME.display()),
        Err(e) => {
            erm!("Failed to clean {}: {}", RIDHOME.display(), e);
            exit(1);
        }
    }

    msg!("All done!")
}

fn mkdir(path: &Path) {
    if !path.exists() {
        if let Err(e) = fs::create_dir_all(path) {
            die!("Failed to create '{}': {}", path.display(), e)
        }
        vpr!("Created directory '{}'", path.display());
    }
}

pub fn tmp() {
    vpr!("Creating temp dirs...");
    let dirs = [&*TMPRID, &*BUILDING, &*EXTRACTION, &*DEST, &*TRASH];

    for dir in dirs.iter() {
        mkdir(dir)
    }
    vpr!("Creating pkgs.json if nonexistent...");
    create_json().expect("Failed to create $RIDPKGSJSON");
}

pub fn run() {
    let dirs = [&*RIDHOME, &*SOURCES, &*META];

    for dir in dirs.iter() {
        mkdir(dir)
    }
    bootstrap();
}
