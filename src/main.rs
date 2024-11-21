// main.rs

use pm::PM;
use sets::handle_sets;
use tracking::load_pkglist;
use package::Package;
use args::init_args;

mod utils;
mod paths;
mod core;
mod misc;
mod resolvedeps;
mod flags;
mod checks;
mod package;
mod macros;
mod tracking;
mod sets;
mod pm;
mod args;

fn main() {
    println!("Hi");

    let args = init_args();

    flags::set_flags(true, args.quiet, args.force);

    let pkglist = load_pkglist();
    let pkgs = args.packages;
    let pkgs = handle_sets(pkgs);
    let pkgs = pkgs.iter().map(|pkg| Package::new(pkg, pkglist.clone())).collect::<Vec<Package>>();

    let mut pm = PM::new(pkgs, pkglist);

    if args.list {
        pm.list()
    }

    if args.cache {
        pm.cache()
    }

    if args.dependencies {
        pm.dependencies()
    }
}
