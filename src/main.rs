// main.rs

use pm::PM;
use sets::handle_sets;
use tracking::load_pkglist;
use package::Package;

mod upstream;
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

    macro_rules! invoke{
        ($args:expr, $pm:expr, [ $( $method:ident ),* $(,)? ]) => {
            $(
                if $args.$method {
                    $pm.$method();
                }
            )*
        };
    }

    invoke!(args, pm, [
        list,
        cache,
        dependencies,
        dependants,
        get,
        remove,
        remove_with_dependencies,
        install,
        install_with_dependencies,
        update,
        update_with_dependencies,
        news,
        prune,
        check_upstream,
    ]);
}
