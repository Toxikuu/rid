// src/flags.rs
//
// stores flags for global use

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref FORCE:   Mutex<bool> = Mutex::new(false);
    pub static ref QUIET:   Mutex<bool> = Mutex::new(false);
    pub static ref VERBOSE: Mutex<bool> = Mutex::new(false);
}

pub fn set_flags(force: bool, quiet: bool, verbose: bool) {
    *FORCE.lock().unwrap()   = force;
    *QUIET.lock().unwrap()   = quiet;
    *VERBOSE.lock().unwrap() = verbose;
}
