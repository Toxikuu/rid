// examples.rs
//
// defines helper functions for examples
#![allow(dead_code)] // these functions are used in the examples, but not in rid

use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use whoami::username;

pub fn e2c() {
    println!("Press enter to continue...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new());
}

// used for examples
pub fn rid_cmd(args: &[&str]) {
    if username() != "root" {
        panic!("Examples must be run as root");
    }

    println!(" $ rid {}\n", args.join(" "));
    sleep(Duration::from_millis(1337));

    let o = Command::new("rid")
        .args(args)
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&o.stdout));
    e2c();
}
