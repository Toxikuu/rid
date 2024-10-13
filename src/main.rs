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
    #[arg(short = 'i', long, value_name = "PACKAGE", value_parser, num_args = 1.., value_delimiter = ' ')]
    install: Option<Vec<String>>,

    #[arg(short = 'n', long, value_name = "PACKAGE", value_parser, num_args = 1.., value_delimiter = ' ')]
    install_no_deps: Option<Vec<String>>,

    #[arg(short = 'r', long, value_name = "PACKAGE", value_parser, num_args = 1.., value_delimiter = ' ')]

    remove: Option<Vec<String>>,

    #[arg(short = 'u', long, value_name = "PACKAGE", value_parser, num_args = 1.., value_delimiter = ' ')]
    update: Option<Vec<String>>,

    #[arg(short = 'd', long, value_name = "PACKAGE", value_parser, num_args = 1.., value_delimiter = ' ')]
    dependencies: Option<Vec<String>>,
    
    // function flags
    #[arg(short = 'l', long)]
    list: bool,

    #[arg(short = 'b', long)]
    bootstrap: bool,

    #[arg(short = 's', long)]
    sync: bool,

    #[arg(short = 'S', long)]
    sync_overwrite: bool,

    // generic flags
    #[arg(short = 'v', long)]
    verbose: bool,

    #[arg(short = 'q', long)]
    quiet: bool,

    #[arg(short = 'D', long)]
    download: bool,

    #[arg(short = 'f', long)]
    force: bool,
}

fn main() {
    let args = Args::parse();
    flags::set_flags(args.verbose, args.quiet, args.download, args.force);
    pr!(format!("Flags: verbose={}, quiet={}, download={}, force={}", 
                args.verbose, args.quiet, args.download, args.force), 'v');

    bootstrap::tmp();
    //let _ = misc::exec("sleep 5");

    let _ = tracking::populate_txt();
    let _ = tracking::align('~');
    let _ = tracking::prune();
    let _ = tracking::alphabetize();

    match args {
        Args { install_no_deps: Some(pkgs), .. } => {
            check_perms();

            for pkg in pkgs {
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
        }
        Args { install: Some(pkgs), .. } => {
            check_perms();

            for pkg in pkgs {
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
        }
        Args { remove: Some(pkgs), .. } => {
            check_perms();

            for pkg in pkgs {
                pr!(format!("Removing package: {}", pkg));
                eval_removal_directions(&pkg);
                tracking::remove_package(&pkg);
            }
        }
        Args { update: Some(pkgs), .. } => {
            check_perms();

            for pkg in pkgs {
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
        }
        Args { dependencies: Some(pkgs), .. } => {

            for pkg in pkgs {
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
        }
        Args { list, .. } if list => {
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
        Args { sync, ..} if sync => {
            check_perms();
            pr!("\x1b[36;1mSyncing rid-meta...\x1b[0m");
            bootstrap::get_rid_meta(false);
        }
        Args { sync_overwrite, ..} if sync_overwrite => {
            check_perms();
            pr!("\x1b[36;1mSyncing rid-meta with overwrite...\x1b[0m");
            bootstrap::get_rid_meta(true);
        }
        _ => {
            pr!("No valid arguments provided.")
        }
    }
}
