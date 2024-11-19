// src/main.rs

use checks::check_perms;
use defargs::init_args;

mod args;
mod bootstrap;
mod checks;
mod clean;
mod defargs;
mod directions;
mod fetch;
mod flags;
mod macros;
mod misc;
mod package;
mod paths;
mod resolvedeps;
mod sets;
mod tracking;

fn main() {
    let args = init_args();
    check_perms();

    flags::set_flags(args.verbose, args.quiet, args.force);
    vpr!(
        "Flags: verbose={}, quiet={}, force={}",
        args.verbose,
        args.quiet,
        args.force
    );

    bootstrap::tmp();

    vpr!("Loading package list...");
    let mut pkg_list = tracking::load_package_list();
    vpr!("Loaded {} packages", pkg_list.len());

    if pkg_list.is_empty() {
        vpr!("Populating empty json!");
        match tracking::cache_changes(&mut pkg_list, true) {
            Ok(num) => vpr!("Populated empty json with {} packages", num),
            Err(e) => die!("Error populating empty json: {}", e),
        }
    }

    if !args.cache {
        vpr!("Autocaching...");
        match tracking::cache_changes(&mut pkg_list, false) {
            Ok(num) => vpr!("Autocached {} packages", num),
            Err(e) => die!("Error autocaching: {}", e),
        }
    }

    if args.bootstrap {
        bootstrap::bootstrap();
    }

    if args.cache {
        args::cache(&mut pkg_list);
    }

    if args.check_upstream {
        args::check_upstream();
    }

    if args.validate_links {
        args::validate_links();
    }

    //if args.sync {
    //    args::sync();
    //}
    //
    //if args.overwrite {
    //    args::overwrite();
    //}

    if let Some(pkgs) = args.list {
        args::list(pkgs);
    }

    if let Some(pkgs) = args.remove {
        args::remove(pkgs, &mut pkg_list, args.force);
    }

    if let Some(pkgs) = args.remove_with_dependencies {
        args::remove_with_dependencies(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.prune {
        args::prune(pkgs);
    }

    if let Some(pkgs) = args.install {
        args::install(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.install_with_dependencies {
        args::install_with_dependencies(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.update {
        args::update(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.update_with_dependencies {
        args::update_with_dependencies(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.dependencies {
        args::dependencies(pkgs);
    }

    if let Some(pkgs) = args.dependants {
        args::dependants(pkgs, &pkg_list);
    }

    if let Some(pkgs) = args.news {
        args::news(pkgs);
    }

    if let Some(pkgs) = args.get_files {
        args::get_files(pkgs, false);
    }

    if let Some(pkgs) = args.force_get_files {
        args::get_files(pkgs, true);
    }
}
