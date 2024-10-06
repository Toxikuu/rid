// src/directions.rs
//
// Responsible for executing various build directions

use crate::misc::exec;
use crate::paths::UTILS;
use crate::tracking::query_status;
use crate::flags::FULL_FORCE;

use crate::pr;

pub fn eval_install_directions(pkg_str: &str) {
    match query_status(pkg_str) {
        Ok(status) => {
            pr!(format!("Status: {}", status), 'v');
            match status {
                "installed" => {
                    if !*FULL_FORCE.lock().unwrap() {
                        pr!(format!("Package '{}' is already installed", pkg_str));
                        return;
                    } else {
                        pr!(format!("Forcibly reinstalling package '{}'", pkg_str));
                    }
                },
                "available" => {},
                _ => {
                    pr!(format!("Package '{}' unavailable", pkg_str));
                    return;
                },
            }
            let command = format!("{}/meta-interface.sh i {}", UTILS.display(), pkg_str);
            if let Err(e) = exec(&command) {
                eprintln!("Failed to evaluate install directions: {}", e);
            }
        },
        Err(e) => eprintln!("Error querying package: {}", e),
    }
}

pub fn eval_removal_directions(pkg_str: &str) {
    match query_status(pkg_str) {
        Ok(status) => {
            pr!(format!("Status: {}", status), 'v');
            match status {
                "installed" => {},
                "available" => {
                    if !*FULL_FORCE.lock().unwrap() {
                        pr!(format!("Package '{}' is not installed", pkg_str));
                        return;
                    } else {
                        pr!(format!("Forcibly removing package '{}'", pkg_str));
                    }
                },
                _ => {
                    pr!(format!("Package '{}' unavailable", pkg_str));
                    return;
                }
            }
            let command = format!("{}/meta-interface.sh r {}", UTILS.display(), pkg_str);
            if let Err(e) = exec(&command) {
                eprintln!("Failed to evaluate removal directions: {}", e);
            }
        },
        Err(e) => eprintln!("Error querying package: {}", e),
    }
}

pub fn eval_update_directions(pkg_str: &str) {
    let command = format!("{}/meta-interface.sh u {}", UTILS.display(), pkg_str);
    if let Err(e) = exec(&command) {
        eprintln!("Failed to evaluate update directions: {}", e);
    }
}
