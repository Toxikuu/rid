// src/directions.rs
//
// responsible for executing various build directions

use crate::flags::FORCE;
use crate::misc::exec;
use crate::package::PackageStatus;
use crate::paths::RBIN;
use crate::tracking::query_status;

use crate::{erm, pr};

pub fn eval_install_directions(pkg_str: &str) {
    match query_status(pkg_str) {
        Ok(status) => {
            pr!(format!("Status: {:?}", status), 'v');

            match status {
                PackageStatus::Installed => {
                    if !*FORCE.lock().unwrap() {
                        pr!(format!("Package '{}' is already installed", pkg_str));
                        return;
                    } else {
                        pr!(format!("Forcibly installing package '{}'", pkg_str), 'v');
                    }
                }
                PackageStatus::Available => {}
                PackageStatus::Removed => {
                    pr!(format!(
                        "Package '{}' has been removed. Reinstalling.",
                        pkg_str
                    ));
                }
            }

            let command = format!("{}/mint i {}", RBIN.display(), pkg_str);
            if let Err(e) = exec(&command) {
                erm!("Failed to evaluate install directions: {}", e);
            }
        }
        Err(e) => erm!("Failed to query package status: {}", e),
    }
}

pub fn eval_removal_directions(pkg_str: &str) {
    match query_status(pkg_str) {
        Ok(status) => {
            pr!(format!("Status: {:?}", status), 'v');
            match status {
                PackageStatus::Installed => {}
                PackageStatus::Available => {
                    if !*FORCE.lock().unwrap() {
                        pr!(format!("Package '{}' is not installed", pkg_str));
                        return;
                    } else {
                        pr!(format!("Forcibly removing package '{}'", pkg_str), 'v');
                    }
                }
                _ => {
                    pr!(format!("Package '{}' unavailable", pkg_str));
                    return;
                }
            }
            let command = format!("{}/mint r {}", RBIN.display(), pkg_str);
            if let Err(e) = exec(&command) {
                erm!("Failed to evaluate removal directions: {}", e);
            }
        }
        Err(e) => erm!("Error querying package: {}", e),
    }
}

pub fn eval_update_directions(pkg_str: &str) {
    let command = format!("{}/mint u {}", RBIN.display(), pkg_str);
    if let Err(e) = exec(&command) {
        erm!("Failed to evaluate update directions: {}", e);
    }
}
