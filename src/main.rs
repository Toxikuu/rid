// main.rs

use package::Package;
use paths::REPO;
use pm::PM;
use sets::handle_sets;
use tracking::load_pkglist;
use utils::pkg_search;

mod args;
mod checks;
mod cmd;
mod config;
mod core;
mod examples;
mod flags;
mod init;
mod linkval;
mod macros;
mod package;
mod paths;
mod pm;
mod resolve;
mod sets;
mod tracking;
mod upstream;
mod utils;

fn main() {
    let args = args::init_args();
    init::init();
    flags::set_flags(args.force, args.quiet, args.verbose);

    vpr!("Set repo to {}", &*REPO);
    let mut pkglist = load_pkglist();
    let pkgs: Vec<Option<String>> = args.packages
        .iter()
        .map(|pkg| {
            if pkg.starts_with('@') { return Some(pkg.to_string()) }
            if pkg.starts_with('^') { return Some(pkg.to_string().strip_prefix('^').unwrap().to_string()) }

            if pkglist.iter().any(|p| p.name == *pkg) {
                Some(pkg.clone())
            } else {
                vpr!("Searching for the closest match for '{}'...", pkg);
                pkg_search(pkg, pkglist.clone())
            }
        })
        .collect();

    let pkgs: Vec<String> = pkgs.into_iter()
        .flatten()
        .collect();

    let pkgs = handle_sets(pkgs, &pkglist);

    let mut cache_list: Vec<String> = Vec::new();
    let mut force_cache = false;
    if args.cache {
        cache_list = pkgs.clone();
        if cache_list.is_empty() { cache_list = pkglist.iter().map(|p| p.name.clone()).collect() }
        msg!("Caching {} packages", cache_list.len());
        force_cache = true;
    } else {
        vpr!("Autocaching...");
    }

    match tracking::cache_changes(force_cache, &mut pkglist, cache_list) {
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
        outdated,
        search,
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
        validate_links,
        sync,
    ]);
}
