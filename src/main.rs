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

    let mut cache_list: Vec<String> = Vec::new();
    if args.cache {
        cache_list = pkgs.clone();
        if cache_list.is_empty() { cache_list = pkglist.iter().map(|p| p.name.clone()).collect() }
        msg!("Caching {} packages", cache_list.len());
    } else {
        vpr!("Autocaching...");
    }

    match tracking::cache_changes(&mut pkglist, cache_list) {
        Ok(n) => vpr!("Cached {} packages", n),
        Err(e) => die!("Error caching: {}", e),
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
