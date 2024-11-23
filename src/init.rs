// init.rs
//
// initializes rid

use crate::checks::check_perms;
use crate::tracking;
use crate::die;
use crate::utils::mkdir;
use crate::paths::{
    TMPRID, BUILDING, EXTRACTION, DEST, TRASH
};

fn tmp() {
    let dirs = [&*TMPRID, &*BUILDING, &*EXTRACTION, &*DEST, &*TRASH];

    for dir in dirs.iter() { mkdir(dir) }
    if tracking::create_json().is_err() {
        die!("Failed to create json")
    }
}

pub fn init() {
    check_perms();
    tmp()
}
