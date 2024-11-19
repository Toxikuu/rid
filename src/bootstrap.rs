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

fn bootstrap() {
    let dirs = [&*RIDHOME, &*SOURCES, &*META];

    for dir in dirs.iter() {
        mkdir(dir)
    }

    let _ = down(
        "https://raw.githubusercontent.com/Toxikuu/rid/refs/heads/master/install.sh",
        true,
    );

    let command = "bash \"$RIDSOURCES\"/install.sh";
    let _ = exec(command);
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
