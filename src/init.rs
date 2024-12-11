// init.rs
//
// initializes rid

use crate::checks::check_perms;
use crate::die;
use crate::tracking;
use crate::paths::{TMPRID, BUILDING, EXTRACTION, DEST, TRASH};
use crate::utils::mkdir;

fn tmp() {
    let dirs = [&*BUILDING, &*DEST, &*EXTRACTION, &*TMPRID, &*TRASH];

    for dir in dirs.iter() { mkdir(dir) }
    if tracking::create_json().is_err() {
        die!("Failed to create json")
    }
}

pub fn init() {
    check_perms();
    tmp()
}
