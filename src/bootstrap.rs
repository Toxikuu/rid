// src/bootstrap.rs
//
// responsible for bootsrapping rid

use crate::paths::*;
use crate::{erm, vpr};
use std::fs;
use std::io;
use std::path::Path;

#[cfg(not(feature = "offline"))]
mod online {
    pub use crate::misc::exec;
    pub use crate::msg;
    pub use crate::tracking::populate_json;
    pub use std::error::Error;
    pub use std::fs::File;
    pub use std::io::Write;
    pub use std::process::exit;
    pub use ureq::get;
}

#[cfg(not(feature = "offline"))]
use online::*;

#[cfg(not(feature = "offline"))]
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

#[cfg(not(feature = "offline"))]
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

#[cfg(not(feature = "offline"))]
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

#[cfg(not(feature = "offline"))]
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

fn mkdir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path_ref = path.as_ref();

    if path_ref.exists() {
        vpr!("Extant directory '{}'", path_ref.display());
    } else {
        fs::create_dir_all(path_ref)?;
        vpr!("Created directory '{}'", path_ref.display());
    }
    Ok(())
}

pub fn tmp() {
    vpr!("Attempting to create temp dirs...");
    let dirs = [&*TMPRID, &*BUILDING, &*EXTRACTION, &*DEST, &*TRASH];

    for dir in dirs.iter() {
        if let Err(e) = mkdir(dir) {
            erm!("Error creating directory: {}", e);
        }
    }
}

#[cfg(not(feature = "offline"))]
pub fn run() {
    let dirs = [&*RIDHOME, &*SOURCES, &*META];

    for dir in dirs.iter() {
        if let Err(e) = mkdir(dir) {
            erm!("Error creating directory: {}", e);
        }
    }

    let _ = populate_json();
    bootstrap();
}
