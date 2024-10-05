// src/tracking.rs

use std::fs::{File, OpenOptions};
use regex::Regex;
use std::io::{self, BufRead, BufWriter, Write};
use crate::paths::{PKGSTXT, META};
use crate::misc::{self, list_dir};
use crate::package;

pub fn align_tildes() -> io::Result<()> {
    let file = File::open(&*PKGSTXT)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    let mut max_tilde_pos = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some(pos) = line.find('~') {
            if pos > max_tilde_pos {
                max_tilde_pos = pos;
            }
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&*PKGSTXT)?;

    for line in lines {
        if let Some(pos) = line.find('~') {
            let padding = max_tilde_pos.saturating_sub(pos);
            let (before_tilde, after_tilde) = line.split_at(pos);
            writeln!(file, "{}{}{}", before_tilde, " ".repeat(padding), after_tilde)?;
        } else {
            writeln!(file, "{}", line)?;
        }
    }

    Ok(())
}

pub fn is_package_installed(string: &str, pkg_str: &str) -> bool {
    let pattern = format!(r"{}-\d+.+~ installed", regex::escape(pkg_str));
    let re = Regex::new(&pattern).unwrap();

    re.is_match(string.trim())
}

pub fn add_package(pkg_str: &str) {
    let path = META.to_str().expect("Invalid UTF-8");
    match list_dir(path) {
        Ok(files) => {
            //println!("Files in directory '{}':", path);
            //for file in &files {
            //    println!("{}", file);
            //}

            if files.iter().any(|f| f.to_lowercase() == pkg_str) {
                
                match misc::read_file(PKGSTXT.clone()) {
                    Ok(contents) => {
                        //if tracked_packages.iter().any(|pkg| pkg.name.to_lowercase() == inp) {
                        if is_package_installed(&contents, pkg_str) {
                            println!("Package '{}' is already tracked.", pkg_str);
                            return;
                        }

                        match package::form_package(pkg_str) {
                            Ok(pkg) => {
                                match package::track_package(pkg) {
                                    Ok(()) => println!("Successfully tracked package."),
                                    Err(e) => eprintln!("Failed to track package: {}", e),
                                }

                                match align_tildes() {
                                    Ok(()) => println!("Successfully formatted packages.txt."),
                                    Err(e) => eprintln!("Failed to format tracking file: {}", e),
                                }
                            },
                            Err(e) => eprintln!("Failed to form package: {}", e)
                        }
                    },
                    Err(e) => eprintln!("Error reading packages file: {}", e),
                }
            } else {
                eprintln!("Error: '{}' not found in meta directory.", pkg_str)
            }
        },
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
}

pub fn remove_package(pkg_str: &str) -> io::Result<()> {
    let file = File::open(&*PKGSTXT)?;
    let reader = io::BufReader::new(file);
    let mut removed = false;
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let pattern = format!("{}-", pkg_str);
        if line.contains(&pattern) && line.contains("~ installed") {
            println!("Untracking package: {}", pkg_str);
            //let modified_line = line.replace("~ installed", "~ available");
            //lines.push(modified_line);
            removed = true;
            continue;
        }
        //} else {
        //    lines.push(line);
        //}
        lines.push(line)
    }

    if ! removed {
        println!("Package '{}' not tracked.", pkg_str);
    }

    let file = File::create(&*PKGSTXT)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
    
}
