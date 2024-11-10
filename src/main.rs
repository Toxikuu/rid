// src/main.rs

use checks::check_perms;
use defargs::init_args;
use tracking::populate_json;

mod checks;
mod sets;
mod args;
mod defargs;
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

fn main() {
    let args = init_args();
    check_perms(); // for sake of simplicity
    // TODO: Either use `shopt login_shell` to check for login shell or work around the
    // inconsistencies otherwise

    flags::set_flags(args.verbose, args.quiet, args.download, args.force);
    vpr!(
        "Flags:\nverbose={}\nquiet={}\ndownload={}\nforce={}",
        args.verbose,
        args.quiet,
        args.download,
        args.force
    );

    bootstrap::tmp();

    vpr!("Loading pkg_list...");
    let mut pkg_list = tracking::load_package_list();
    vpr!("Loaded! (Length: {})", pkg_list.len());

    if pkg_list.is_empty() {
        populate_json().unwrap();
    }

    if !args.cache {
        vpr!("Autocaching...");
        match tracking::cache_changes(&mut pkg_list) {
            Ok(num) => vpr!("Autocached {} meta files", num),
            Err(e) => die!("Error autocaching: {}", e)
        }
    }

    if args.bootstrap {
        args::bootstrap();
    }

    if args.cache {
        args::cache(&mut pkg_list);
    }

    if args.upstream {
        args::upstream();
    }

    if args.validate_links {
        args::validate_links();
    }

    if args.sync {
        args::sync();
    }

    if args.sync_overwrite {
        args::sync_overwrite();
    }

    if let Some(pkgs) = args.list {
        args::list(pkgs);
    }

    if let Some(pkgs) = args.remove {
        args::remove(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.prune {
        args::prune(pkgs);
    }

    if let Some(pkgs) = args.install_no_deps {
        args::install_no_deps(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.install {
        args::install(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.update {
        args::update(pkgs, &mut pkg_list);
    }

    if let Some(pkgs) = args.dependencies {
        args::dependencies(pkgs);
    }
}
