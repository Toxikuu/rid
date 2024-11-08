// src/misc.rs
//
// defines miscellaneous helper functions

use crate::package::Package;
use crate::paths::{TMPRID, PKGSJSON};
use crate::{erm, vpr, pr};
use serde_json::from_str;
use std::fs::{self, File, read_to_string, OpenOptions as OO};
use std::time::SystemTime;
use std::path::Path;
use std::io::{self, BufRead, Write};
use std::process::{self, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use whoami::username;

pub fn check_perms() {
    if username() != "root" {
        erm!("Insufficient privileges!");
        process::exit(1);
    }
}

pub fn get_mod_time(path: &Path) -> io::Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    metadata.modified().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn is_file_empty(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(m) => m.len() == 0,
        Err(_) => true,
    }
}

pub fn create_json() -> io::Result<()> {
    if !is_file_empty(&PKGSJSON) {
        return Ok(())
    }

    let mut file = File::create(&*PKGSJSON)?;
    file.write_all(b"[]")?;
    vpr!("Wrote [] to empty pkgs.json");

    Ok(())
}

pub fn format_line(line: &str, max_length: usize) -> String {
    let parts: Vec<&str> = line.split('~').collect();

    if parts.len() < 2 {
        return line.to_string();
    }

    let package_info = parts[0].trim();
    let status = parts[1].trim();
    let formatted_status = match status {
        "Available" => "\x1b[30mAvailable\x1b[0m".to_string(),
        "Installed" => "\x1b[36;1mInstalled\x1b[0m".to_string(),
        _ => status.to_string(),
    };

    let name_version_length = package_info.len() + 1;

    let padding = if max_length > name_version_length {
        max_length - name_version_length
    } else {
        0
    };
    let spaces = " ".repeat(padding);

    format!("{}{} ~ {}", package_info, spaces, formatted_status)
}

pub fn static_exec(command: &str) -> io::Result<String> {
    let output = Command::new("bash").arg("-c").arg(command).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

pub fn exec(command: &str) -> io::Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let log_file = Arc::new(Mutex::new(
        OO::new()
            .append(true)
            .create(true)
            .open(format!("{}/rid.log", TMPRID.display()))
            .expect("Failed to open log file"),
    ));

    let log_file_stdout = Arc::clone(&log_file);
    let stdout_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    pr!("{}", line);
                    let log_line = format!("{}\n", line);
                    let mut log_file = log_file_stdout.lock().unwrap();
                    let _ = write!(log_file, "{}", log_line);
                }
                Err(e) => erm!("Error reading stdout: {}", e),
            }
        }
    });

    let log_file_stderr = Arc::clone(&log_file);
    let stderr_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    pr!("\x1b[31;1;3m{}", line); // override default formatting for pr!
                    let log_line = format!("[ERR] {}\n", line);
                    let mut log_file = log_file_stderr.lock().unwrap();
                    let _ = write!(log_file, "{}", log_line);
                }
                Err(e) => erm!("Error reading stderr: {}", e),
            }
        }
    });

    let _ = child.wait()?;
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    Ok(())
}

pub fn read_pkgs_json() -> Result<Vec<Package>, String> {
    let contents = read_to_string(&*PKGSJSON).map_err(|e| e.to_string())?;
    from_str(&contents).map_err(|e| e.to_string())
}
