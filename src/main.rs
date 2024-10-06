// src/main.rs

use clap::Parser;
use directions::{eval_install_directions, eval_removal_directions, eval_update_directions};
use misc::check_perms;
use crate::paths::PKGSTXT;

mod package;
mod tracking;
mod paths;
mod misc;
mod fetch;
mod directions;
mod resolvedeps;
mod bootstrap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, value_name = "PACKAGE")]
    install: Option<String>,

    #[arg(short = 'I', long, value_name = "PACKAGE")]
    install_no_deps: Option<String>,

    #[arg(short = 'r', long, value_name = "PACKAGE")]
    remove: Option<String>,

    #[arg(short = 'l', long)]
    list: bool,

    #[arg(short = 'u', long, value_name = "PACKAGE")]
    update: Option<String>,

    #[arg(short = 'd', long, value_name = "PACKAGE")]
    dependencies: Option<String>,

    #[arg(short = 'b', long)]
    bootstrap: bool,
}

fn main() {
    let args = Args::parse();

    match args {
        Args { install_no_deps: Some(pkg), .. } => {
            check_perms();
            println!("Installing package: {}", pkg);
            match package::form_package(&pkg) {
                Ok(pkg_) => {
                    fetch::wrap(&pkg_);
                    eval_install_directions(&pkg);
                    tracking::add_package(&pkg, &pkg_.version);
                },
                Err(e) => eprintln!("Failed to form package '{}': {}", pkg, e),
            }
        }
        Args { install: Some(pkg), .. } => {
            check_perms();
            println!("Installing package '{}' with dependencies", pkg);

            match package::form_package(&pkg) {
                Ok(pkg_) => {
                    let dependencies = resolvedeps::resolve_deps(&pkg_);
                    for dep in &dependencies {
                        println!(" - {}", dep);
                    }

                    for dep in dependencies {
                        match package::form_package(&dep) {
                            Ok(dep_) => {
                                fetch::wrap(&dep_);
                                eval_install_directions(&dep);
                                tracking::add_package(&dep, &dep_.version);
                            },
                            Err(e) => eprintln!("Failed to form package '{}': {}", dep, e),
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to form package '{}': {}", pkg, e);
                },
            }
        }
        Args { remove: Some(pkg), .. } => {
            check_perms();
            println!("Removing package: {}", pkg);
            eval_removal_directions(&pkg);
            tracking::remove_package(&pkg);
        }
        Args { update: Some(pkg), .. } => {
            check_perms();
            println!("Updating package: {}", pkg);

            match package::form_package(&pkg) {
                Ok(pkg_) => {
                    fetch::wrap(&pkg_);
                    eval_update_directions(&pkg);
                    tracking::add_package(&pkg, &pkg_.version);
                },
                Err(e) => eprintln!("Failed to form package '{}': {}", pkg, e),
            }
        }
        Args { dependencies: Some(pkg), .. } => {
            println!("Dependencies for {}:", pkg);
            match package::form_package(&pkg) {
                Ok(pkg_) => {
                    let dependencies = resolvedeps::resolve_deps(&pkg_);
                    for dep in dependencies {
                        println!(" - {}", dep);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to form package '{}': {}", pkg, e);
                },
            }
        }
        Args { list, .. } if list => {
            check_perms();

            let _ = tracking::populate_txt();
            let _ = tracking::align('~');
            let _ = tracking::alphabetize();

            match misc::read_file(PKGSTXT.clone()) {
                Ok(contents) => {
                    println!("\x1b[30;1;3mPACKAGES\x1b[0m");
                    for line in contents.lines() {
                        let formatted_line = misc::format_line(line);
                        println!("  {}", formatted_line);
                    }
                },
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
        Args { bootstrap, ..} if bootstrap => {
            check_perms();
            println!("\x1b[36;1mBootstrapping rid...\x1b[0m");
            bootstrap::run();
        }
        _ => {
            println!("No valid arguments provided.")
        }
    }
}
