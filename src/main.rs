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
mod flags;
mod macros;

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

    // generic flags
    #[arg(short = 'v', long)]
    verbose: bool,

    #[arg(short = 'q', long)]
    quiet: bool,

    #[arg(short = 'f', long)]
    force: bool,

    #[arg(short = 'F', long)]
    full_force: bool,
}

fn main() {
    let args = Args::parse();
    flags::set_flags(args.verbose, args.quiet, args.force, args.full_force);
    pr!(format!("Flags: verbose={}, quiet={}, force={}, full_force={}", 
                args.verbose, args.quiet, args.force, args.full_force), 'v');

    bootstrap::tmp();

    match args {
        Args { install_no_deps: Some(pkg), .. } => {
            check_perms();
            pr!(format!("Installing package: {}", pkg));
            pr!("Installing without dependencies", 'v');
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
            pr!(format!("Installing package '{}'", pkg), 'v');

            match package::form_package(&pkg) {
                Ok(pkg_) => {
                    let dependencies = resolvedeps::resolve_deps(&pkg_);
                    for dep in &dependencies {
                        pr!(format!(" - {}", dep));
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
            pr!(format!("Removing package: {}", pkg));
            eval_removal_directions(&pkg);
            tracking::remove_package(&pkg);
        }
        Args { update: Some(pkg), .. } => {
            check_perms();
            pr!(format!("Updating package: {}", pkg));

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
            pr!(format!("Dependencies for {}:", pkg));
            match package::form_package(&pkg) {
                Ok(pkg_) => {
                    let dependencies = resolvedeps::resolve_deps(&pkg_);
                    for dep in dependencies {
                        pr!(format!(" - {}", dep), 'q');
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
                    pr!("\x1b[30;1;3mPACKAGES\x1b[0m");
                    for line in contents.lines() {
                        let formatted_line = misc::format_line(line);
                        pr!(format!("  {}", formatted_line), 'q');
                    }
                },
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
        Args { bootstrap, ..} if bootstrap => {
            check_perms();
            pr!("\x1b[36;1mBootstrapping rid...\x1b[0m");
            bootstrap::run();
        }
        _ => {
            pr!("No valid arguments provided.")
        }
    }
}
