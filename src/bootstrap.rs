// src/bootstrap.rs
//
// Responsible for bootsrapping rid
// Bootsrapping includes installing directories into /etc/rid and grabbing meta files

use std::fs::{self, File};
use std::path::Path;
use std::io;
use crate::paths::{SOURCES, META, UTILS, PKGSTXT};

fn mkdir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path_ref = path.as_ref();

    if path_ref.exists() {
        println!("Directory '{}' extant", path_ref.display());
    } else {
        fs::create_dir_all(path_ref)?;
        println!("Created directory '{}'", path_ref.display());
    }

    Ok(())
}

fn touch<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path_ref = path.as_ref();

    if path_ref.exists() {
        println!("File '{}' extant", path_ref.display());
    } else {
        let _file = File::create(path_ref)?;
        println!("Created file '{}'", path_ref.display());
    }

    Ok(())
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
