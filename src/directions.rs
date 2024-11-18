// src/directions.rs
//
// responsible for executing various build directions

use crate::die;
use crate::misc::exec;
use crate::package::Package;
use crate::paths::BIN;

pub fn eval_action(a: char, p: &Package) {
    let command = format!("{}/mint {} {}", BIN.display(), a, p.name);
    if let Err(e) = exec(&command) {
        die!("Failed to evaluate action '{}': {}", a, e)
    }
}
