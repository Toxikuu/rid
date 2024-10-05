// src/main.rs

use clap::Parser;
use crate::paths::PKGSTXT;

mod package;
mod tracking;
mod paths;
mod misc;
mod fetch;
mod directions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, value_name = "PACKAGE")]
    install: Option<String>,

    #[arg(short = 'r', long, value_name = "PACKAGE")]
    remove: Option<String>,

    #[arg(short = 'l', long)]
    list: bool,
}

fn main() {
    let args = Args::parse();

    match args {
        Args { install: Some(pkg), .. } => {
            misc::check_perms();
            println!("Installing package: {}", pkg);
            match package::form_package(&pkg) {
                Ok(pkg) => {
                    let pkgname = &pkg.name.clone();
                    fetch::wrap(pkg);
                    directions::eval_install_directions(pkgname);
                    tracking::add_package(pkgname);
                },
                Err(e) => eprintln!("Failed to form package '{pkg}': {}", e),
            }
        }
        Args { remove: Some(pkg), .. } => {
            misc::check_perms();
            println!("Removing package: {}", pkg);
            directions::eval_removal_directions(&pkg);
            let _ = tracking::remove_package(&pkg);
        }
        Args { list, .. } if list => {
            match misc::read_file(PKGSTXT.clone()) {
                Ok(contents) => println!("{}", contents),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
        _ => {
            println!("No valid arguments provided.")
        }
    }
}
