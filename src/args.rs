// args.rs
//
// responsible for storing argument functions

use indicatif::{ProgressBar, ProgressStyle};
use tracking::read_pkgs_json;
use crate::flags::FORCE;
use crate::options::split_opts;
use crate::{die, erm, msg, vpr, pr, yn};
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

#[cfg(not(feature = "offline"))]
pub fn bootstrap() {
    msg!("Bootstrapping rid...");
    bootstrap::run();
}

pub fn cache(pkg_list: &mut Vec<Package>) {
    msg!("Caching meta files to json...");
    match tracking::cache_changes(pkg_list, true) {
        Ok(num) => msg!("Cached {} packages!", num),
        Err(e) => erm!("Failed to cache: {}", e)
    }
}

#[cfg(not(feature = "offline"))]
pub fn sync() {
    msg!("Syncing rid-meta...");
    bootstrap::get_rid_meta(false);
}

#[cfg(not(feature = "offline"))]
pub fn overwrite() {
    msg!("Overwrite-syncing rid-meta...");
    bootstrap::get_rid_meta(true);
}

fn display_list(mut plist: Vec<Package>) {
    plist.sort_by(|a, b| a.name.cmp(&b.name));

    for p in &plist {
        let mut iv = "";
        if let PackageStatus::Installed = p.status {
            iv = &p.installed_version;    
        }

        let line = format!("{}={} ~ {:?} {}", p.name, p.version, p.status, iv);
        let formatted_line = misc::format_line(&line, 32);
        println!("  {}", formatted_line);
    }
    vpr!("Displayed {} packages", plist.len())
}

pub fn list(mut pkgs: Vec<String>) {
    if pkgs.is_empty() {
        pkgs.push("@all".to_string());
    }

    let mut displayed = Vec::new();
    let pkgs = handle_sets(pkgs);

    // read_pkgs_json is far more performant than calling defp for pkg in pkgs
    let all_pkgs = match read_pkgs_json() {
        Ok(j) => j,
        Err(e) => die!("Failed to read $RIDPKGSJSON: {}", e)
    };

    for pkg in pkgs {
        let (pkg, _) = split_opts(&pkg);
        if let Some(pkg_data) = all_pkgs.iter().find(|p| p.name == pkg) {
            displayed.push(pkg_data.clone())
        } else {
            die!("Package '{}' not found in $RIDPKGSJSON", pkg);
        }
    }

    msg!("PACKAGES");
    display_list(displayed);
}

fn dedup(mut vec: Vec<Package>) -> Vec<Package> {
    vec.sort_by(|a, b| a.name.cmp(&b.name));
    vec.dedup();
    vec
}

fn find_dependants(pkg: &str, pkg_list: &[Package]) -> Vec<Package> {
    let mut dependants: Vec<Package> = Vec::new();
    for p in pkg_list.iter() {
        if p.deps.contains(&pkg.to_string()) {
            vpr!("Found dependant package: '{}'", p.name);
            if !dependants.contains(p) {
                dependants.push(p.clone());
            }
        }
    }
    
    vpr!("Found {} dependants", dependants.len());
    dependants
}

pub fn remove(pkgs: Vec<String>, pkg_list: &mut Vec<Package>, force: bool) {
    for pkg in handle_sets(pkgs) {
        if !force {
            vpr!("Checking dependants for '{}'", pkg);

            let mut dependants = find_dependants(&pkg, pkg_list);
            dependants.retain(|p| p.name != *pkg);
            dependants = dedup(dependants);
            vpr!("Found {} dependants", dependants.len());
            if !dependants.is_empty() {
                let message = format!("Remove '{}' ({} dependants)?", pkg, dependants.len());
                if !yn!(&message, false) {
                    vpr!("Aborting removal since 'n' was selected");
                    return
                }
            }
        }

        let p = defp(&pkg);
        msg!("Removing {}-{}", p.name, p.version);
        eval_action('r', &p);
        match tracking::remove_package(pkg_list, &pkg) {
            Ok(_) => msg!("Removed {}", pkg),
            Err(e) => {
                erm!("Failed to track package '{}': {}", pkg, e);
            }
        }
        clean::remove_tarballs(&pkg);
    }
}

pub fn remove_with_dependencies(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    // recursively removes a package and all its dependencies
    // this can be very dangerous if removal instructions are implemented for core packages
    let force = *FORCE.lock().unwrap();
    let pkgs = handle_sets(pkgs);

    for pkg in pkgs.iter() {
        if force { erm!("WARNING: Skipping all checks for dependants of '{}'", pkg) }
        if !force { vpr!("Checking for dependants of '{}' and its dependencies", pkg) }

        let pkg = defp(pkg);
        let deps = resolve_deps(&pkg);
        display_deps(&deps, pkg.clone());
        
        let mut dependants: Vec<Package> = Vec::new();
        for dep in deps.iter() {
            if force {
                remove(deps, pkg_list, true);
                return
            }
            dependants.extend(find_dependants(dep, pkg_list));
            dependants.retain(|p| p.name != *dep);
            vpr!("Removed redundant dependants")
        }

        dependants = dedup(dependants);
        vpr!("Deduplicated dependants list");
        if dependants.is_empty() {
            vpr!("Proceeding with removal since no dependants were found");
        } else {
            erm!("Found {} dependant packages:", dependants.len());
            display_list(dependants.clone());
            let message = format!("Remove '{}' and its dependencies ({} total dependants)?", pkg.name, dependants.len());
            if !yn!(&message, false) {
                vpr!("Aborting removal since 'n' was selected");
                return
            }
        }

        remove(deps, pkg_list, true)
    }
}

const TEMPLATE: &str =
    "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {pos}/{len} ({eta})";

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
    eval_action('i', &p);
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
    for pkg in handle_sets(pkgs) {
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
            // if dep.is_empty() {
            //     die!("Undefined dependency detected");
            // }

            for pkg in &plist {
                if pkg.name == *dep {
                    matches.push(format!(
                        "{}={} ~ {:?} {}",
                        pkg.name, pkg.version, pkg.status, pkg.installed_version
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
    for pkg in handle_sets(pkgs) {
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
    for pkg in handle_sets(pkgs) {
        let p = defp(&pkg);

        if p.installed_version == p.version && !*FORCE.lock().unwrap() {
            msg!("Package '{}' up to date", p.name);
            continue
        }

        msg!("Updating {}", p.name);
        fetch::fetch(&p);
        eval_action('u', &p);

        match tracking::add_package(pkg_list, &p) {
            Ok(_) => msg!("Updated to {}-{}", p.name, p.version),
            Err(e) => erm!("Failed to track package '{}': {}", p.name, e),
        }
    }
}

pub fn update_with_dependencies(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    for pkg in handle_sets(pkgs) {
        let p = defp(&pkg);

        let deps = resolve_deps(&p);
        display_deps(&deps, p);

        for dep in deps {
            let d = match form_package(&dep) {
                Ok(d) => d,
                Err(e) => { erm!("{}", e); return },
            };

            if d.installed_version == d.version && !*FORCE.lock().unwrap() {
                msg!("Package '{}' up to date", d.name);
                continue
            }

            msg!("Updating {}", d.name);
            fetch::fetch(&d);
            eval_action('u', &d);

            match tracking::add_package(pkg_list, &d) {
                Ok(_) => msg!("Updated to {}-{}", d.name, d.version),
                Err(e) => erm!("Failed to track package '{}': {}", d.name, e),
            }
        }
    }
}

pub fn dependencies(pkgs: Vec<String>) {
    for pkg in handle_sets(pkgs) {
        let p = defp(&pkg);

        let deps = resolve_deps(&p);
        display_deps(&deps, p);
    }
}

pub fn dependants(pkgs: Vec<String>, pkg_list: &[Package]) {
    for pkg in handle_sets(pkgs) {
        let mut dependants = find_dependants(&pkg, pkg_list);
        dependants.retain(|p| p.name != *pkg);
        vpr!("Removed redundant dependants");
        if dependants.is_empty() {
            msg!("No dependants for {}", pkg);
            return;
        }
        msg!("Dependants for {}", pkg);
        display_list(dependants);
    }
}

pub fn news(pkgs: Vec<String>) {
    for pkg in handle_sets(pkgs) {
        let p = defp(&pkg);

        vpr!("Checking for news for package '{}'", p.name);
        vpr!("News: {}", p.news);
        if !p.news.is_empty() {
            msg!("News for {}:", p.name);
            pr!("\x1b[31;3m{}\x1b[0m\n", p.news);
        }
    }
}

#[cfg(not(feature = "offline"))]
pub fn get_tarball(pkgs: Vec<String>) {
    for pkg in handle_sets(pkgs) {
        let p = defp(&pkg);
        if let Err(e) = fetch::download(&p, true) {
            die!("Failed to fetch tarball for package '{}' with link '{}': {}", p.name, p.link, e);
        }
    }
}

#[cfg(not(feature = "offline"))]
pub fn check_upstream() {
    if let Err(e) = exec("stab") {
        die!("Failed to check upstream: {}", e)
    }
}

#[cfg(not(feature = "offline"))]
pub fn validate_links() {
    if let Err(e) = exec("linkval") {
        die!("Failed to validate links: {}", e)
    }
}
