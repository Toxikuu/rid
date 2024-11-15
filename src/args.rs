// args.rs
//
// responsible for storing argument functions

use indicatif::{ProgressBar, ProgressStyle};
use tracking::read_pkgs_json;
use crate::flags::FORCE;
use crate::{die, erm, msg, vpr};
use crate::tracking;
use crate::misc;
use crate::clean;
use crate::package::*;
use crate::directions::eval_action;
use crate::fetch;
use crate::resolvedeps::resolve_deps;
use crate::sets::*;

#[cfg(not(feature = "offline"))]
mod online {
    pub use crate::misc::exec;
    pub(crate) use crate::bootstrap;
}

#[cfg(not(feature = "offline"))]
use online::*;

#[cfg(feature = "offline")]
fn not_supported(feature: &str) {
    die!("{} is not supported for rid-offline", feature);
}

pub fn bootstrap() {
    #[cfg(feature = "offline")]
    not_supported("Bootstrapping");

    #[cfg(not(feature = "offline"))]
    {
        msg!("Bootstrapping rid...");
        bootstrap::run();
    }
}

pub fn cache(pkg_list: &mut Vec<Package>) {
    msg!("Caching meta files to json...");
    if !*FORCE.lock().unwrap() {
        match tracking::cache_changes(pkg_list, false) {
            Ok(num) => msg!("Cached {} meta files!", num),
            Err(e) => erm!("Failed to cache: {}", e)
        }
    } else {
        match tracking::cache_changes(pkg_list, true) {
            Ok(num) => msg!("Cached {} meta files!", num),
            Err(e) => erm!("Failed to cache: {}", e)
        }
    }
}

pub fn sync() {
    #[cfg(feature = "offline")]
    not_supported("Sync");

    #[cfg(not(feature = "offline"))]
    {
        msg!("Syncing rid-meta...");
        bootstrap::get_rid_meta(false);
    }
}

pub fn overwrite() {
    #[cfg(feature = "offline")]
    not_supported("Sync");

    #[cfg(not(feature = "offline"))]
    {
        msg!("Overwrite-syncing rid-meta...");
        bootstrap::get_rid_meta(true);
    }
}

fn display_list(p_list: Vec<Package>) {
    msg!("PACKAGES");
    for p in &p_list {
        let mut iv = "";
        if let PackageStatus::Installed = p.status {
            iv = &p.installed_version;    
        }

        let line = format!("{}={} ~ {:?} {}", p.name, p.version, p.status, iv);
        let formatted_line = misc::format_line(&line, 32);
        println!("  {}", formatted_line);
    }
    vpr!("Displayed {} packages", p_list.len())
}

pub fn list(pkgs: Vec<String>) {
    if pkgs.is_empty() {
        match read_pkgs_json() {
            Ok(p_list) => {
                display_list(p_list)
            }
            Err(e) => erm!("Errror reading pkgs.json: {}", e),
        }
    } else {
        let mut p_list = Vec::new();
        let pkgs = handle_sets(pkgs);

        let all_pkgs = match read_pkgs_json() {
            Ok(j) => j,
            Err(e) => {
                erm!("Error reading $RIDPKGSJSON: {}", e);
                return;
            }
        };

        for pkg in pkgs {
            if let Some(pkg_data) = all_pkgs.iter().find(|p| p.name == pkg) {
                p_list.push(pkg_data.clone())
            } else {
                erm!("Package '{}' not found in $RIDPKGSJSON", pkg);
            }
        }

        display_list(p_list);
    }
}

pub fn remove(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    let pkgs = handle_sets(pkgs);

    for pkg in pkgs {
        msg!("Removing {}", pkg);
        eval_action('r', &pkg);
        match tracking::remove_package(pkg_list, &pkg) {
            Ok(_) => msg!("Removed {}", pkg),
            Err(e) => {
                erm!("Failed to track package '{}': {}", pkg, e);
            }
        }
        clean::remove_tarballs(&pkg);
    }
}

const TEMPLATE: &str =
    "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {completed}/{total} ({eta})";

pub fn prune(pkgs: Vec<String>) {
    let pkgs = handle_sets(pkgs);
    let mut tarballs_removed = 0;

    let length = pkgs.len() as u64;
    let bar = ProgressBar::new(length);

    bar.set_message("Pruning packages");
    bar.set_style(ProgressStyle::with_template(TEMPLATE).unwrap().progress_chars("#|-"));
    bar.set_length(length);

    for pkg in pkgs {
        vpr!("Pruning {}", pkg);
        
        let p = defp(&pkg);
        let num_removed = clean::prune_sources(&p);
        vpr!("Pruned {} tarballs for {}", num_removed, pkg);
        tarballs_removed += num_removed;
        bar.inc(1);
    }

    let message = format!("Pruned {} tarballs for {} packages", tarballs_removed, length);
    bar.finish_with_message(message);
}

fn wrap_install(p: Package, pkg_list: &mut Vec<Package>) {
    fetch::fetch(&p);
    eval_action('i', &p.name);
    match tracking::add_package(pkg_list, &p) {
        Ok(_) => msg!("Installed {}-{}", p.name, p.version),
        Err(e) => die!("Failed to track package '{}': {}", p.name, e),
    }
}

fn do_install(p: &Package, extra: &str) -> bool {
    // whether to perform an install
    match p.status {
        PackageStatus::Installed => {
            msg!("{}-{} is already installed", p.name, p.version);
            *FORCE.lock().unwrap()
        }
        _ => {
            msg!("Installing {}-{} {}", p.name, p.version, extra);
            true
        }
    }
}


pub fn install(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    let pkgs = handle_sets(pkgs);

    for pkg in pkgs {
        let p = defp(&pkg);

        if do_install(&p, "without dependencies") {
            wrap_install(p, pkg_list)
        }
    }
}

fn display_deps(deps: &Vec<String>, p: Package) {
    msg!("Dependencies for {}:", p.name);

    if let Ok(plist) = read_pkgs_json() {
        let mut matches: Vec<String> = Vec::new();

        for dep in deps {
            if dep.is_empty() {
                die!("Undefined dependency detected");
            }

            for pkg in &plist {
                if pkg.name == *dep {
                    matches.push(format!(
                        "{}={} ~ {:?}",
                        pkg.name, pkg.version, pkg.status
                    ))
                }
            }
        }

        for m in matches {
            println!("  {}", misc::format_line(&m, 32));
        }    
    }
}

pub fn install_with_dependencies(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    let pkgs = handle_sets(pkgs);

    for pkg in pkgs {
        let p = defp(&pkg);
        let deps = resolve_deps(&p);
        display_deps(&deps, p);

        for dep in deps {
            let d = match form_package(&dep) {
                Ok(d) => d,
                Err(e) => { erm!("{}", e); return },
            };

            if do_install(&d, "") {
                wrap_install(d, pkg_list)
            }
        }
    }
}

pub fn update(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    let pkgs = handle_sets(pkgs);

    for pkg in pkgs {
        let p = defp(&pkg);

        if p.installed_version == p.version && !*FORCE.lock().unwrap() {
            msg!("Package '{}' up to date", p.name);
            continue
        }

        msg!("Updating {}", p.name);
        fetch::fetch(&p);
        eval_action('u', &p.name);

        match tracking::add_package(pkg_list, &p) {
            Ok(_) => msg!("Updated to {}-{}", p.name, p.version),
            Err(e) => erm!("Failed to track package '{}': {}", pkg, e),
        }
    }
}

pub fn dependencies(pkgs: Vec<String>) {
    let pkgs = handle_sets(pkgs);

    for pkg in pkgs {
        let p = defp(&pkg);

        let deps = resolve_deps(&p);
        display_deps(&deps, p);
    }
}

pub fn check_upstream() {
    #[cfg(feature = "offline")]
    not_supported("Upstream checking");

    #[cfg(not(feature = "offline"))]
    if let Err(e) = exec("stab") {
        die!("Failed to check upstream: {}", e)
    }
}

pub fn validate_links() {
    #[cfg(feature = "offline")]
    not_supported("Link validation");

    #[cfg(not(feature = "offline"))]
    if let Err(e) = exec("linkval") {
        die!("Failed to validate links: {}", e)
    }
}
