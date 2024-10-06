// src/directions.rs
//
// Responsible for executing various build directions

use crate::misc::exec;
use crate::paths::UTILS;

pub fn eval_install_directions(pkg_str: &str) {
    let command = format!("{}/meta-interface.sh i {}", UTILS.display(), pkg_str);
    if let Err(e) = exec(&command) {
        eprintln!("Failed to evaluate install directions: {}", e);
    }
    //match exec(&command) {
    //    Ok(output) => println!("{}", output),
    //    Err(e) => eprintln!("Failed to evaluate install directions: {}", e),
    //}
}


pub fn eval_removal_directions(pkg_str: &str) {
    let command = format!("{}/meta-interface.sh r {}", UTILS.display(), pkg_str);
    if let Err(e) = exec(&command) {
        eprintln!("Failed to evaluate removal directions: {}", e);
    }
    //match exec(&command) {
    //    Ok(output) => println!("{}", output),
    //    Err(e) => eprintln!("Failed to evaluate removal directions: {}", e),
    //}
}


pub fn eval_update_directions(pkg_str: &str) {
    let command = format!("{}/meta-interface.sh u {}", UTILS.display(), pkg_str);
    if let Err(e) = exec(&command) {
        eprintln!("Failed to evaluate update directions: {}", e);
    }
    //match exec(&command) {
    //    Ok(output) => println!("{}", output),
    //    Err(e) => eprintln!("Failed to evaluate update directions: {}", e),
    //}
}
