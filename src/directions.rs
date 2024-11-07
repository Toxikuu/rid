// src/directions.rs
//
// responsible for executing various build directions

use crate::die;
use crate::misc::exec;
use crate::paths::BIN;

pub fn eval_action(a: char, pkg_str: &str) {
    let command = format!("{}/mint {} {}", BIN.display(), a, pkg_str);
    if let Err(e) = exec(&command) {
        die!("Failed to evaluate action '{}': {}", a, e)
    }
}
