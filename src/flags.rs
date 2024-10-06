// src/flags.rs

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref VERBOSE: Mutex<bool>    = Mutex::new(false);
    pub static ref QUIET: Mutex<bool>      = Mutex::new(false);
    pub static ref FORCE_DOWNLOAD: Mutex<bool>      = Mutex::new(false);
    pub static ref FORCE_INSTALL: Mutex<bool> = Mutex::new(false);
    pub static ref FORCE_REMOVE: Mutex<bool> = Mutex::new(false);
}

pub fn set_flags(verbose: bool, quiet: bool, force_download: bool, force_install: bool, force_remove: bool) {
    *VERBOSE.lock().unwrap() = verbose;
    *QUIET.lock().unwrap() = quiet;
    *FORCE_DOWNLOAD.lock().unwrap() = force_download;
    *FORCE_INSTALL.lock().unwrap() = force_install;
    *FORCE_REMOVE.lock().unwrap() = force_remove;
}
