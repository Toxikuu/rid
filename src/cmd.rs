// src/misc.rs
//
// defines functions related to command execution

use crate::paths::TMPRID;
use crate::{erm, pr};
use std::fs::OpenOptions as OO;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

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
                    pr!("\x1b[31;1;3m{}", line);
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
