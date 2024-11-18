// src/misc.rs
//
// defines miscellaneous helper functions

use crate::paths::TMPRID;
use crate::{erm, pr};
use std::fs::{self, OpenOptions as OO};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

pub fn get_mod_time(path: &Path) -> io::Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    metadata
        .modified()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn format_line(line: &str, max_length: usize) -> String {
    let parts: Vec<&str> = line.split('~').collect();

    if parts.len() < 2 {
        return line.to_string();
    }

    let package_info = parts[0].trim();
    let status = parts[1].trim();
    let formatted_status = if status.contains("Available") {
        format!("\x1b[30m{}\x1b[0m", status)
    } else if status.contains("Installed") {
        format!("\x1b[36;1m{}\x1b[0m", status)
    } else {
        unreachable!("Invalid status for format_line()")
    };

    let name_version_length = package_info.len() + 1;
    let padding = max_length.saturating_sub(name_version_length); // thank you rust-analyzer :))
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
