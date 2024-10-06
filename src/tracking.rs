// src/tracking.rs

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufWriter, Write};
use std::collections::HashSet;
use crate::paths::{PKGSTXT, META};
use crate::package::form_package;

use crate::pr;

pub fn alphabetize() -> io::Result<()> {
    let file = File::open(&*PKGSTXT)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    lines.sort_by(|a, b| {
        let pkg_a = a.split('=').next().unwrap_or("").trim();
        let pkg_b = b.split('=').next().unwrap_or("").trim();

        pkg_a.cmp(pkg_b)
    });

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&*PKGSTXT)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn align(c: char) -> io::Result<()> {
    let file = File::open(&*PKGSTXT)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    let mut max_tilde_pos = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some(pos) = line.find(c) {
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
        if let Some(pos) = line.find(c) {
            let (before_tilde, after_tilde) = line.split_at(pos);
            let padding = max_tilde_pos.saturating_sub(pos);

            writeln!(file, "{}{}{}", before_tilde, " ".repeat(padding), after_tilde)?;
        } else {
            writeln!(file, "{}", line)?;
        }
    }

    Ok(())
}

pub fn populate_txt() -> io::Result<()> {
    let file = File::open(&*PKGSTXT).or_else(|_| File::create(&*PKGSTXT))?;
    let reader = io::BufReader::new(file);

    let mut existing_packages = HashSet::new();
    for line in reader.lines() {
        let line_content = line?;
        if let Some(pkg_name) = line_content.split('=').next() {
            existing_packages.insert(pkg_name.trim().to_string());
        }
    }

    let mut pkgstxt = OpenOptions::new().append(true).open(&*PKGSTXT)?;
    for entry in fs::read_dir(&*META)? {
        if let Ok(entry) = entry {
            let pkg_name = entry.file_name().into_string().unwrap_or_default();

            if !pkg_name.is_empty() && !existing_packages.contains(&pkg_name) {
                match form_package(&pkg_name) {
                    Ok(pkg) => {
                        writeln!(pkgstxt, "{}={} ~ available", pkg_name, pkg.version)?;
                        pr!(format!("Added new package: {}={}", pkg_name, pkg.version), 'v');
                    },
                    Err(e) => eprintln!("Failed to form package: {}", e)
                }
            }
        } else {
            eprintln!("Failed to read directory entry");
        }
    }

    Ok(())
}

pub fn remove_package(pkg_str: &str) {
    let file = File::open(&*PKGSTXT).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut removed = false;

    for line in reader.lines() {
        let line = line.unwrap();

        let pattern = format!("{}=", pkg_str);

        if line.contains(&pattern) && line.contains("~ installed") {
            pr!(format!("Untracking package: {}", pkg_str), 'v');
            let modified_line = line.replace("~ installed", "~ available");
            lines.push(modified_line);
            removed = true;
        } else {
            lines.push(line);
        }
    }

    if ! removed {
        pr!(format!("Package '{}' is already removed.", pkg_str));
    }

    let file = File::create(&*PKGSTXT).unwrap();
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line).unwrap();
    }
}

pub fn add_package(pkg_str: &str, vers: &str) {
    let file = File::open(&*PKGSTXT).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut installed = false;
    let mut modified = false;

    for line in reader.lines() {
        let line = line.unwrap();

        let pattern = format!("{}=", pkg_str);

        if line.contains(&pattern) {
            if line.contains("~ installed") {
                installed = true;
                lines.push(line);
            } else if line.contains("~ available") {
                pr!(format!("Tracking package: {}", pkg_str), 'v');
                let modified_line = format!("{}={} ~ installed", pkg_str, vers);
                lines.push(modified_line);
                installed = true;
                modified = true;
            }
        } else {
            lines.push(line);
        }
    }


    if modified {
        pr!(format!("Package '{}' has been installed", pkg_str), 'v');
    } else if installed {
        pr!(format!("Package '{}' is already installed", pkg_str), 'v');
    } else {
        pr!(format!("Package '{}' is not available", pkg_str), 'v');
    }

    let file = File::create(&*PKGSTXT).unwrap();
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line).unwrap();
    }
}
