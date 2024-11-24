// main.rs

use pm::PM;
use sets::handle_sets;
use tracking::load_pkglist;
use package::Package;

mod init;
mod utils;
mod paths;
mod core;
mod cmd;
mod resolve;
mod flags;
mod checks;
mod package;
mod macros;
mod tracking;
mod sets;
mod pm;
mod args;

fn main() {
    let args = args::init_args();
    init::init();
    flags::set_flags(args.verbose, args.quiet, args.force);


    let pkgs = args.packages;
    let mut pkglist = load_pkglist();
    let pkgs = handle_sets(pkgs, &pkglist);

    // could probably be a pm method(?)
    if !args.cache {
        vpr!("Autocaching...");
        match tracking::cache_changes(&mut pkglist, false) {
            Ok(num) => vpr!("Autocached {} packages", num),
            Err(e) => die!("Error autocaching: {}", e),
        }
    }

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

    if args.dependants {
        pm.dependants()
    }

    if args.get {
        pm.get()
    }

    if args.remove {
        pm.remove()
    }

    if args.install {
        pm.install()
    }

    if args.install_with_dependencies {
        pm.install_with_dependencies()
    }

    if args.update {
        pm.update()
    }

    if args.update_with_dependencies {
        pm.update_with_dependencies()
    }

    if args.news {
        pm.news()
    }

    if args.prune {
        pm.prune()
    }
}
