// src/directions.rs
//
// responsible for executing various build directions

use crate::erm;
use crate::misc::exec;
use crate::paths::RBIN;

pub fn eval_install_directions(pkg_str: &str) {
    let command = format!("{}/mint i {}", RBIN.display(), pkg_str);
    if let Err(e) = exec(&command) {
        erm!("Failed to evaluate install directions: {}", e);
    }
}

pub fn eval_removal_directions(pkg_str: &str) {
    let command = format!("{}/mint r {}", RBIN.display(), pkg_str);
    if let Err(e) = exec(&command) {
        erm!("Failed to evaluate removal directions: {}", e);
    }
}

pub fn eval_update_directions(pkg_str: &str) {
    let command = format!("{}/mint u {}", RBIN.display(), pkg_str);
    if let Err(e) = exec(&command) {
        erm!("Failed to evaluate update directions: {}", e);
    }
}
