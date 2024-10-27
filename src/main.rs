// src/main.rs

use crate::flags::FORCE;
use crate::paths::PKGSJSON;
use clap::Parser;
use directions::{eval_install_directions, eval_removal_directions, eval_update_directions};
use misc::check_perms;
use package::{form_package, PackageStatus};
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
    let mut args = Args::parse();
    let mut matched = false;

    if args.update.is_some() {
        args.force = true;
    }

    flags::set_flags(args.verbose, args.quiet, args.download, args.force);
    vpr!(
        "Flags:\nverbose={}\nquiet={}\ndownload={}\nforce={}",
        args.verbose,
        args.quiet,
        args.download,
        args.force
    );

    bootstrap::tmp();
    let mut pkg_list =
        tracking::load_package_list(PKGSJSON.as_path()).unwrap_or_else(|_| Vec::new());
    let _ = tracking::append_json(&mut pkg_list); // appends any new metafiles to the json

    if args.bootstrap {
        matched = true;
        #[cfg(feature = "offline")]
        {
            erm!("Bootstrapping is not supported for offline rid");
            exit(1)
        }

        #[cfg(not(feature = "offline"))]
        {
            check_perms();
            msg!("Bootstrapping rid...");
            bootstrap::run();
        }
    }

    if args.cache {
        matched = true;
        let _ = tracking::populate_json();
    }

    if args.sync {
        matched = true;
        #[cfg(feature = "offline")]
        {
            erm!("Syncing is not supported for offline rid");
            exit(1)
        }

        #[cfg(not(feature = "offline"))]
        {
            check_perms();
            msg!("Syncing rid-meta...");
            bootstrap::get_rid_meta(false);
        }
    }

    if args.sync_overwrite {
        matched = true;
        #[cfg(feature = "offline")]
        {
            erm!("Syncing is not supported for offline rid");
            exit(1)
        }

        #[cfg(not(feature = "offline"))]
        {
            check_perms();
            msg!("Syncing rid-meta with overwrite...");
            bootstrap::get_rid_meta(true);
        }
    }

    if args.list {
        matched = true;
        match misc::read_json(PKGSJSON.clone()) {
            Ok(package_list) => {
                msg!("PACKAGES");
                for p in package_list {
                    let line = format!("{}={} ~ {:?}", p.name, p.version, p.status,);

                    let formatted_line = misc::format_line(&line, 32);
                    println!("  {}", formatted_line);
                }
            }
            Err(e) => erm!("Errror reading pkgs.json: {}", e),
        }
    }

    if let Some(pkgs) = args.remove {
        matched = true;
        check_perms();
        for pkg in pkgs {
            msg!("Removing {}", pkg);
            eval_removal_directions(&pkg);
            match tracking::remove_package(&mut pkg_list, &pkg) {
                Ok(_) => msg!("Removed {}", pkg),
                Err(e) => {
                    erm!("Failed to track package '{}': {}", pkg, e);
                }
            }
            clean::remove_tarballs(&pkg);
        }
    }

    if let Some(pkgs) = args.prune {
        matched = true;
        check_perms();

        for pkg in pkgs {
            msg!("Pruning {}", pkg);
            match form_package(&pkg) {
                Ok(p) => clean::prune_sources(&p),
                Err(e) => erm!("{}", e),
            }
        }
    }

    if let Some(pkgs) = args.install_no_deps {
        matched = true;
        check_perms();

        for pkg in pkgs {
            match package::form_package(&pkg) {
                Ok(p) => {
                    let mut do_install = false;

                    match &p.status {
                        PackageStatus::Installed => {
                            msg!("{}-{} is already installed", p.name, p.version);
                            if *FORCE.lock().unwrap() {
                                do_install = true
                            };
                        }
                        _ => {
                            msg!("Installing {}-{} without dependencies", p.name, p.version);
                            do_install = true;
                        }
                    }

                    vpr!("do_install = {}", do_install);
                    if do_install {
                        fetch::wrap(&p);
                        eval_install_directions(&p.name);
                        match tracking::add_package(&mut pkg_list, &p) {
                            Ok(_) => msg!("Installed {}-{}", p.name, p.version),
                            Err(e) => {
                                erm!("Failed to track package '{}': {}", pkg, e);
                                exit(1);
                            }
                        }
                    }
                }
                Err(e) => erm!("{}", e),
            }
        }
    }

    if let Some(pkgs) = args.install {
        matched = true;
        check_perms();

        for pkg in pkgs {
            match package::form_package(&pkg) {
                Ok(p) => {
                    let deps = resolvedeps::resolve_deps(&p);
                    for dep in &deps {
                        vpr!("Dependency: {}", dep);
                    }

                    // i may want to add a function for displaying dependencies and share it
                    // between --dependencies and here

                    for dep in deps {
                        match package::form_package(&dep) {
                            Ok(d) => {
                                let mut do_install = false;

                                match &d.status {
                                    PackageStatus::Installed => {
                                        msg!("{}-{} is already installed", d.name, d.version);
                                        if *FORCE.lock().unwrap() {
                                            do_install = true
                                        };
                                    }
                                    _ => {
                                        msg!("Installing {}-{}", d.name, d.version);
                                        do_install = true;
                                    }
                                }

                                vpr!("do_install = {}", do_install);
                                if do_install {
                                    fetch::wrap(&d);
                                    eval_install_directions(&dep);
                                    match tracking::add_package(&mut pkg_list, &d) {
                                        Ok(_) => msg!("Installed {}-{}", d.name, d.version),
                                        Err(e) => {
                                            erm!("Failed to track package '{}': {}", dep, e)
                                        }
                                    }
                                }
                            }
                            Err(e) => erm!("{}", e),
                        }
                    }
                }
                Err(e) => {
                    erm!("{}", e);
                }
            }
        }
    }

    if let Some(pkgs) = args.update {
        matched = true;
        check_perms();

        for pkg in pkgs {
            msg!("Updating {}", pkg);

            match package::form_package(&pkg) {
                Ok(p) => {
                    fetch::wrap(&p);
                    eval_update_directions(&p.name);
                    match tracking::add_package(&mut pkg_list, &p) {
                        Ok(_) => msg!("Updated to {}-{}", p.name, p.version),
                        Err(e) => {
                            erm!("Failed to track package '{}': {}", pkg, e);
                        }
                    }
                }
                Err(e) => erm!("{}", e),
            }
        }
    }

    if let Some(pkgs) = args.dependencies {
        matched = true;
        for pkg in pkgs {
            msg!("Dependencies for {}:", pkg);

            match package::form_package(&pkg) {
                Ok(p) => {
                    let deps = resolvedeps::resolve_deps(&p);

                    match misc::read_json(PKGSJSON.clone()) {
                        Ok(package_list) => {
                            let mut matches: Vec<String> = Vec::new();

                            for dep in &deps {
                                if dep.is_empty() {
                                    erm!("Undefined dependency detected!");
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
                                println!("  {}", formatted_m)
                            }
                        }
                        Err(e) => {
                            erm!("Error reading packages.json: {}", e);
                        }
                    }
                }
                Err(e) => {
                    erm!("{}", e);
                }
            }
        }
    }

    if !matched {
        erm!("No valid arguments provided")
    }
}
