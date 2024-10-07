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
use std::os::unix::fs::PermissionsExt;
use crate::paths::{SOURCES, META, UTILS, PKGSTXT};

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
        "https://raw.githubusercontent.com/Toxikuu/rid/refs/heads/master/utils/meta-interface.sh",
        "/etc/rid/utils"
    ) {
        Ok(_) => pr!("Downloaded meta-interface", 'v'),
        Err(e) => {
            eprintln!("Failed to download meta-interface.sh: {}", e);
            exit(1);
        }
    }
    match dl(
        "https://raw.githubusercontent.com/Toxikuu/rid/refs/heads/master/meta/rid",
        "/etc/rid/meta"
        ) {
        Ok(_) => {
            pr!("Downloaded rid");
            pr!("Now, run rid -i rid to finish bootstrapping");
        },
        Err(e) => {
            eprintln!("Failed to download rid: {}", e);
            exit(1);
        }
    }

    match dl(
        "https://raw.githubusercontent.com/Toxikuu/rid/refs/heads/master/env",
        "/etc/rid/"
        ) {
        Ok(_) => {
            pr!("Downloaded env", 'v');
        },
        Err(e) => {
            eprintln!("Failed to download env: {}", e);
            exit(1);
        }
    }

    let permissions = fs::Permissions::from_mode(0o755);
    fs::set_permissions("/etc/rid/utils/meta-interface.sh", permissions).unwrap();
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

fn touch<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path_ref = path.as_ref();

    if path_ref.exists() {
        pr!(format!("File '{}' extant", path_ref.display()), 'v');
    } else {
        let _file = File::create(path_ref)?;
        pr!(format!("Created file '{}'", path_ref.display()));
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
    let dirs = [&*SOURCES, &*META, &*UTILS];
    let files = [&*PKGSTXT];

    for dir in dirs.iter() {
        if let Err(e) = mkdir(dir) {
            eprintln!("Error creating directory: {}", e);
        }
    }

    for file in files.iter() {
        if let Err(e) = touch(file) {
            eprintln!("Error creating file: {}", e);
        }
    }

    let permissions = fs::Permissions::from_mode(0o666);
    fs::set_permissions(&*PKGSTXT, permissions).unwrap();


    get_rid();
}
