// sets.rs
//
// responsible for sets functionality

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::paths::SETS;
use crate::{erm, vpr};

pub fn is_set(pkg: &str) -> bool {
    pkg.contains("@")
}

fn is_comment(pkg: &str) -> bool {
    pkg.contains("/")  ||
    pkg.contains("#")  ||
    pkg.trim().is_empty()
}

pub fn expand_set(set: &str) -> Vec<String> {
    let file_path = format!("{}/{}", SETS.display(), set.replacen('@', "", 1));
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            erm!("Error opening set '{}': {}", set, e);
            return Vec::new();
        }
    };

    let reader = BufReader::new(file);
    let mut all_packages = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(pkg) => {
                let pk = pkg.trim().to_string();
                if is_comment(&pk) { continue }

                if is_set(&pk) {
                    all_packages.extend(expand_set(&pk));
                } else {
                    all_packages.push(pk);
                }
            }
            Err(e) => {
                erm!("Failed to read set: {}", e);
            }
        }
    }
    
    vpr!("unraveled set: {:?}", all_packages);
    all_packages
}
