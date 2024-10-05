// src/misc.rs

use std::io::{self, Read};
use std::fs::{self, File};
use std::process::{self, Command};
use std::path::PathBuf;
use whoami::username;

pub fn check_perms() {
    if username() != "root" {
        eprintln!("Insufficient privileges!");
        process::exit(1);
    }
}

pub fn exec(command: &str) -> io::Result<String> {

    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn list_dir(path: &str) -> Result<Vec<String>, io::Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Ok(name) = entry.file_name().into_string() {
            files.push(name);
        }
    }

    Ok(files)
}

pub fn read_file(file_path: PathBuf) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut contents = String::new();

    io::BufReader::new(file).read_to_string(&mut contents)?;
    Ok(contents)
}

