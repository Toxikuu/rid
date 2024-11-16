// src/flags.rs
//
// stores flags for global use

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref VERBOSE: Mutex<bool> = Mutex::new(false);
    pub static ref QUIET: Mutex<bool> = Mutex::new(false);
    pub static ref FORCE: Mutex<bool> = Mutex::new(false);
}

pub fn set_flags(verbose: bool, quiet: bool, force: bool) {
    *VERBOSE.lock().unwrap() = verbose;
    *QUIET.lock().unwrap() = quiet;
    *FORCE.lock().unwrap() = force;
}
