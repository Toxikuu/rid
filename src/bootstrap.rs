// src/bootstrap.rs
//
// responsible for bootsrapping rid

use crate::paths::*;
use crate::{erm, pr};
use std::fs;
use std::io;
use std::path::Path;

#[cfg(not(feature = "offline"))]
mod online {
    pub use crate::misc::exec;
    pub use crate::msg;
    pub use crate::tracking::populate_json;
    pub use reqwest::blocking::get;
    pub use std::error::Error;
    pub use std::fs::File;
    pub use std::io::Write;
    pub use std::process::exit;
}

#[cfg(not(feature = "offline"))]
use online::*;

#[cfg(not(feature = "offline"))]
fn dl(url: &str, outdir: &str) -> Result<String, Box<dyn Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let file_path = Path::new(outdir).join(file_name);

    let r = get(url)?;
    if r.status().is_success() {
        let mut file = File::create(&file_path)?;

        let content = r.bytes()?;
        file.write_all(&content)?;

        Ok(file_name.to_string())
    } else {
        Err(format!("Failed to download file: HTTP status {}", r.status()).into())
    }
}

#[cfg(not(feature = "offline"))]
fn get_rid() {
    match dl(
        "https://github.com/Toxikuu/rid/archive/refs/heads/master.tar.gz",
        "/tmp/rid/building/",
    ) {
        Ok(_) => pr!("Downloaded rid tarball", 'v'),
        Err(e) => {
            erm!("Failed to download rid tarball: {}", e);
            exit(1);
        }
    }

    match exec(
        "cd       /tmp/rid/building      && \
         tar -xf  master.tar.gz          && \
         mv  -vf  rid-master/* /etc/rid/ && \
         rm  -rf  /tmp/rid/building/*",
    ) {
        Ok(_) => pr!("Set up rid", 'v'),
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
        "/tmp/rid/building/",
    ) {
        Ok(_) => pr!("Downloaded rid-meta tarball", 'v'),
        Err(e) => {
            erm!("Failed to download rid-meta tarball: {}", e);
            exit(1);
        }
    }

    let c = if overwrite { ' ' } else { 'n' };
    let command = format!(
        "cd     /tmp/rid/building                     && \
        tar -xf master.tar.gz                         && \
        rm -f   rid-meta-master/{{LICENSE,README.md}} && \
        mv -v{} rid-meta-master/* /etc/rid/meta/      && \
        rm -rf  master.tar.gz rid-meta-master",
        c
    );

    match exec(&command) {
        Ok(_) => msg!("Synced!"),
        Err(e) => {
            erm!("Failed to sync rid-meta: {}", e);
            exit(1);
        }
    }
}

#[cfg(not(feature = "offline"))]
fn bootstrap() {
    get_rid();
    get_rid_meta(false);

    match exec(
        "touch /etc/rid/pkgs.json               && \
                chmod 666 /etc/rid/pkgs.json    && \
                chmod 755 /etc/rid/rbin/*",
    ) {
        Ok(_) => pr!("Made files in rbin executable", 'v'),
        Err(e) => {
            erm!("Failed to make files in rbin executable: {}", e);
            exit(1);
        }
    }

    // cleanup
    match exec(
        "cd /etc/rid && rm -rf .git* Cargo.* src TDL && \
                cd /etc/rid && rm -rf LICENSE README.md",
    ) {
        Ok(_) => pr!("Cleaned extras from /etc/rid", 'v'),
        Err(e) => {
            erm!("Failed to clean /etc/rid: {}", e);
            exit(1);
        }
    }

    msg!("All done!")
}

fn mkdir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path_ref = path.as_ref();

    if path_ref.exists() {
        pr!(format!("Directory '{}' extant", path_ref.display()), 'v');
    } else {
        fs::create_dir_all(path_ref)?;
        pr!(format!("Created directory '{}'", path_ref.display()), 'v');
    }
    Ok(())
}

pub fn tmp() {
    pr!("Attempting to create temp dirs...", 'v');
    let dirs = [&*TMPRID, &*BUILDING, &*EXTRACTION, &*DEST, &*TRASH];

    for dir in dirs.iter() {
        if let Err(e) = mkdir(dir) {
            erm!("Error creating directory: {}", e);
        }
    }
}

#[cfg(not(feature = "offline"))]
pub fn run() {
    let dirs = [&*ETCRID, &*SOURCES, &*META];

    for dir in dirs.iter() {
        if let Err(e) = mkdir(dir) {
            erm!("Error creating directory: {}", e);
        }
    }

    let _ = populate_json();
    bootstrap();
}
