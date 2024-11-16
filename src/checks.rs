// checks.rs
//
// responsible for checks

use whoami::username;
use crate::die;
use std::fs;
use std::path::Path;

pub fn check_perms() {
    if username() != "root" {
        die!("Rid must be run as root");
    }
}

pub fn is_file_empty(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(m) => m.len() == 0,
        Err(_) => true,
    }
}
