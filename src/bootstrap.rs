// src/bootstrap.rs
//
// Responsible for bootsrapping rid
// Bootsrapping includes creating directories and files
// Plans exist to fetch build scripts as well

use std::fs::{self, File};
use std::path::Path;
use std::io;
use crate::paths::{SOURCES, META, UTILS, PKGSTXT};

use crate::pr;

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
    let dirs = ["/tmp/rid/building", "/tmp/rid/extraction"];

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
}
