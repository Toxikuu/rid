// src/bootstrap.rs
//
// Responsible for bootsrapping rid
// Bootsrapping includes creating directories and files
// Plans exist to fetch build scripts as well

use reqwest::blocking::get;
use std::error::Error;
use std::process::exit;
use std::fs::{self, File};
use std::path::Path;
use std::io::{self, Write};
use crate::paths::{SOURCES, ETCRID};
use crate::misc::exec;

use crate::pr;

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

fn get_rid() {
    match dl(
        "https://github.com/Toxikuu/rid/archive/refs/heads/master.tar.gz",
        "/etc/"
    ) {
        Ok(_) => pr!("Downloaded rid tarball"),
        Err(e) => { eprintln!("Failed to download rid tarball: {}", e); exit(1); }
    }

    match exec("cd /etc && rm -rvf rid; \
                tar xf master.tar.gz && \
                mv -v rid-master rid && \
                rm -vf master.tar.gz") {
        Ok(_) => pr!("Successfully set up rid"),
        Err(e) => { eprintln!("Failed to set up rid: {}", e); exit(1); }
    }

    match dl(
        "https://github.com/Toxikuu/rid-meta/archive/refs/heads/master.tar.gz",
        "/etc/rid/"
    ) {
        Ok(_) => pr!("Downloaded rid-meta tarball"),
        Err(e) => { eprintln!("Failed to download rid-meta tarball: {}", e); exit(1); }
    }

    match exec("cd /etc/rid                && \
                tar xf master.tar.gz       && \
                mv -v rid-meta-master meta && \
                rm -vf master.tar.gz") {
        Ok(_) => pr!("Successfully set up rid-meta"),
        Err(e) => { eprintln!("Failed to set up rid-meta: {}", e); exit(1); }
    }

    match exec("touch /etc/rid/packages.txt     && \
                chmod 666 /etc/rid/packages.txt && \
                chmod 755 /etc/rid/rbin/*") {
        Ok(_) => pr!("Successfully made files in rbin executable"),
        Err(e) => { eprintln!("Failed to make files in rbin executable: {}", e); exit(1); }
    }

    // cleanup
    match exec("cd /etc/rid      && rm -rvf .git* Cargo.* src TDL && \
                cd /etc/rid      && rm -rvf LICENSE README.md     && \
                cd /etc/rid/meta && rm -rvf LICENSE README.md") {
        Ok(_) => pr!("Successfully cleaned /etc/rid"),
        Err(e) => { eprintln!("Failed to clean /etc/rid: {}", e); exit(1); }
    }

    pr!("\x1b[36;1m  All done!\x1b[0m")
}

fn mkdir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path_ref = path.as_ref();

    if path_ref.exists() {
        pr!(format!("Directory '{}' extant", path_ref.display()), 'v');
    } else {
        fs::create_dir_all(path_ref)?;
        pr!(format!("Created directory '{}'", path_ref.display()));
    }
    Ok(())
}

pub fn tmp() {
    pr!("Attempting to create temp dirs...", 'v');
    let dirs = ["/tmp/rid/building", "/tmp/rid/extraction", "/tmp/rid/trash"];

    for dir in dirs.iter() {
        if let Err(e) = mkdir(dir) {
            eprintln!("Error creating directory: {}", e);
        }
    }
}

pub fn run() {
    let dirs = [&*SOURCES, &*ETCRID];

    for dir in dirs.iter() {
        if let Err(e) = mkdir(dir) {
            eprintln!("Error creating directory: {}", e);
        }
    }

    get_rid();
}
