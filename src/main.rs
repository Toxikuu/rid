// src/main.rs

use crate::paths::PKGSJSON;
use clap::Parser;
use directions::{eval_install_directions, eval_removal_directions, eval_update_directions};
use misc::check_perms;
use package::form_package;

#[cfg(feature = "offline")]
use std::process::exit;

mod bootstrap;
mod clean;
mod directions;
mod fetch;
mod flags;
mod macros;
mod misc;
mod package;
mod paths;
mod resolvedeps;
mod tracking;

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

    #[arg(short = 'p', long, value_name = "PACKAGE", value_parser, num_args = 1.., value_delimiter = ' ')]
    prune: Option<Vec<String>>,

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

    #[arg(short = 'c', long)]
    cache: bool,
}

fn main() {
    let args = Args::parse();
    flags::set_flags(args.verbose, args.quiet, args.download, args.force);
    pr!(
        format!(
            "Flags: verbose={}, quiet={}, download={}, force={}",
            args.verbose, args.quiet, args.download, args.force
        ),
        'v'
    );

    bootstrap::tmp();
    let mut pkg_list =
        tracking::load_package_list(PKGSJSON.as_path()).unwrap_or_else(|_| Vec::new());
    let _ = tracking::append_json(&mut pkg_list);

    match args {
        Args { cache, .. } if cache => {
            let _ = tracking::populate_json();
        }

        Args {
            install_no_deps: Some(pkgs),
            ..
        } => {
            check_perms();

            for pkg in pkgs {
                pr!(format!("Installing package: {}", pkg));
                pr!("Installing without dependencies", 'v');
                match package::form_package(&pkg) {
                    Ok(pkg_) => {
                        fetch::wrap(&pkg_);
                        eval_install_directions(&pkg);
                        match tracking::add_package(&mut pkg_list, &pkg) {
                            Ok(_) => pr!(format!(
                                "\x1b[36;1mInstalled {}-{}\x1b[0m",
                                &pkg, &pkg_.version
                            )),
                            Err(e) => eprintln!("Failed to track package '{}': {}", &pkg, e),
                        }
                    }
                    Err(e) => eprintln!("Failed to form package '{}': {}", pkg, e),
                }
            }
        }

        Args {
            install: Some(pkgs),
            ..
        } => {
            check_perms();

            for pkg in pkgs {
                pr!(format!("Installing package '{}'", pkg), 'v');

                match package::form_package(&pkg) {
                    Ok(pkg_) => {
                        let dependencies = resolvedeps::resolve_deps(&pkg_);
                        for dep in &dependencies {
                            pr!(format!("  {}", dep));
                        }

                        for dep in dependencies {
                            match package::form_package(&dep) {
                                Ok(dep_) => {
                                    fetch::wrap(&dep_);
                                    eval_install_directions(&dep);
                                    let _ = tracking::add_package(&mut pkg_list, &dep);
                                }
                                Err(e) => eprintln!("Failed to form package '{}': {}", dep, e),
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to form package '{}': {}", pkg, e);
                    }
                }
            }
        }

        Args {
            remove: Some(pkgs), ..
        } => {
            check_perms();

            for pkg in pkgs {
                pr!(format!("Removing package: {}", pkg));
                eval_removal_directions(&pkg);
                let _ = tracking::remove_package(&mut pkg_list, &pkg);
            }
        }

        Args {
            prune: Some(pkgs), ..
        } => {
            check_perms();

            for pkg in pkgs {
                pr!(format!("Pruning package: {}", pkg));

                match form_package(&pkg) {
                    Ok(p) => clean::sources(p.name, p.version),
                    Err(e) => eprintln!("Failed to form package '{}': {}", pkg, e),
                }
            }
        }

        Args {
            update: Some(pkgs), ..
        } => {
            check_perms();

            for pkg in pkgs {
                pr!(format!("Updating package: {}", pkg));

                match package::form_package(&pkg) {
                    Ok(pkg_) => {
                        fetch::wrap(&pkg_);
                        eval_update_directions(&pkg);
                        let _ = tracking::add_package(&mut pkg_list, &pkg);
                    }
                    Err(e) => eprintln!("Failed to form package '{}': {}", pkg, e),
                }
            }
        }

        Args {
            dependencies: Some(pkgs),
            ..
        } => {
            for pkg in pkgs {
                pr!(format!("\x1b[30;1;3mDependencies for {}:\x1b[0m", pkg));

                match package::form_package(&pkg) {
                    Ok(pkg_) => {
                        let dependencies = resolvedeps::resolve_deps(&pkg_);

                        match misc::read_json(PKGSJSON.clone()) {
                            Ok(package_list) => {
                                let mut matches: Vec<String> = Vec::new();

                                for dep in &dependencies {
                                    if dep.is_empty() {
                                        eprintln!("Undefined dependency detected!");
                                        std::process::exit(1);
                                    }

                                    for package in &package_list {
                                        if package.name == *dep {
                                            matches.push(format!(
                                                "{}={} ~ {:?}",
                                                package.name, package.version, package.status
                                            ))
                                        }
                                    }
                                }

                                for m in matches {
                                    let formatted_m = misc::format_line(&m, 30);
                                    pr!(format!("  {}", formatted_m), 'q')
                                }
                            }
                            Err(e) => {
                                eprintln!("Error reading packages.json: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to form package '{}': {}", pkg, e);
                    }
                }
            }
        }

        Args { list, .. } if list => match misc::read_json(PKGSJSON.clone()) {
            Ok(package_list) => {
                pr!("\x1b[30;1;3mPACKAGES\x1b[0m");
                for package in package_list {
                    let line = format!(
                        "{}={} ~ {:?}",
                        package.name, package.version, package.status,
                    );
                    let formatted_line = misc::format_line(&line, 30);
                    pr!(format!("  {}", formatted_line), 'q');
                }
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        },

        Args { bootstrap, .. } if bootstrap => {
            #[cfg(feature = "offline")]
            {
                pr!("\x1b[36;1mBootstrapping is not supported for offline rid\x1b[0m");
                exit(1)
            }

            #[cfg(not(feature = "offline"))]
            {
                check_perms();
                pr!("\x1b[36;1mBootstrapping rid...\x1b[0m");
                bootstrap::run();
            }
        }

        Args { sync, .. } if sync => {
            #[cfg(feature = "offline")]
            {
                pr!("\x1b[36;1mSyncing is not supported for offline rid\x1b[0m");
                exit(1)
            }

            #[cfg(not(feature = "offline"))]
            {
                check_perms();
                pr!("\x1b[36;1mSyncing rid-meta...\x1b[0m");
                bootstrap::get_rid_meta(false);
            }
        }

        Args { sync_overwrite, .. } if sync_overwrite => {
            #[cfg(feature = "offline")]
            {
                pr!("\x1b[36;1mSyncing is not supported for offline rid\x1b[0m");
                exit(1)
            }

            #[cfg(not(feature = "offline"))]
            {
                check_perms();
                pr!("\x1b[36;1mSyncing rid-meta with overwrite...\x1b[0m");
                bootstrap::get_rid_meta(true);
            }
        }

        _ => {
            pr!("No valid arguments provided.", 'q')
        }
    }
}
