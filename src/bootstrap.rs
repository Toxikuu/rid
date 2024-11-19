// src/bootstrap.rs
//
// responsible for bootsrapping rid

use crate::fetch::down;
use crate::misc::exec;
use crate::paths::*;
use crate::tracking::create_json;
use crate::{die, msg, vpr};
use std::fs;
use std::path::Path;

fn get_rid() {
    let link = format!(
        "https://github.com/Toxikuu/rid/releases/download/v{}/rid-root.tar.xz",
        env!("CARGO_PKG_VERSION")
    );

    let _ = down(&link, true);
    vpr!("Downloaded rid-root tarball");

    let command = format!(
        "tar xf {}/rid-root.tar.xz -C {}",
        BUILDING.display(),
        RIDHOME.display()
    );

    match exec(&command) {
        Ok(_) => vpr!("Extracted rid-root"),
        Err(e) => die!("Failed to extract rid-root tarball: {}", e),
    }
}

pub fn get_rid_meta(overwrite: bool) {
    // used for bootstrapping and syncing
    let link = "https://github.com/Toxikuu/rid-meta/archive/refs/heads/master.tar.gz";
    let _ = down(link, true);
    vpr!("Downloaded rid-meta tarball");

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
        Err(e) => die!("Failed to sync rid-meta: {}", e),
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
            die!("Failed to make files in bin executable: {}", e);
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
            die!("Failed to clean {}: {}", RIDHOME.display(), e);
        }
    }

    msg!("All done!")
}

fn mkdir(path: &Path) {
    if path.exists() {
        return;
    }

    if let Err(e) = fs::create_dir_all(path) {
        die!("Failed to create '{}': {}", path.display(), e)
    }

    vpr!("Created directory '{}'", path.display());
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
