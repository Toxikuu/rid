// checks.rs
//
// responsible for checks

use std::process;
use whoami::username;
use crate::erm;
use std::fs;
use std::path::Path;

pub fn check_perms() {
    if username() != "root" {
        erm!("Insufficient privileges!");
        process::exit(1);
    }
}

pub fn is_file_empty(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(m) => m.len() == 0,
        Err(_) => true,
    }
}
