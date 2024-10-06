// src/misc.rs

use std::thread;
use std::io::{self, BufRead, Read};
use std::fs::File;
use std::process::{self, Command, Stdio};
use std::path::PathBuf;
use whoami::username;

use crate::pr;

pub fn check_perms() {
    if username() != "root" {
        eprintln!("Insufficient privileges!");
        process::exit(1);
    }
}

pub fn format_line(line: &str) -> String {
    match line {
        _ if line.contains("available") => line.replace("available", "\x1b[30mavailable\x1b[0m"),
        _ if line.contains("installed") => line.replace("installed", "\x1b[36;1minstalled\x1b[0m"),
        _ => line.to_string(),
    }
}

pub fn static_exec(command: &str) -> io::Result<String> {

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

pub fn exec(command: &str) -> io::Result<()> {
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    pr!(format!("\x1b[30;3m{}\x1b[0m", line));
                }
                Err(e) => eprintln!("Error reading stdout: {}", e),
            }
        }
    });

    let stderr_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    pr!(format!("\x1b[36;1m{}\x1b[0m", line));
                }
                Err(e) => eprintln!("Error reading stderr: {}", e),
            }
        }
    });

    let _ = child.wait()?;

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    Ok(())
}

pub fn read_file(file_path: PathBuf) -> io::Result<String> {
    let file = File::open(file_path)?;
    let mut contents = String::new();

    io::BufReader::new(file).read_to_string(&mut contents)?;
    Ok(contents)
}

