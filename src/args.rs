// args.rs
//
// responsible for storing argument functions

use crate::bootstrap;
use crate::clean;
use crate::directions::eval_action;
use crate::fetch;
use crate::flags::FORCE;
use crate::misc;
use crate::misc::exec;
use crate::package::*;
use crate::resolvedeps::resolve_deps;
use crate::sets::*;
use crate::tracking;
use crate::{die, erm, msg, pr, vpr, yn};
use indicatif::{ProgressBar, ProgressStyle};
use tracking::read_pkgs_json;

pub fn bootstrap() {
    msg!("Bootstrapping rid...");
    bootstrap::run();
}

pub fn cache(pkg_list: &mut Vec<Package>) {
    msg!("Caching meta files to json...");
    match tracking::cache_changes(pkg_list, true) {
        Ok(num) => msg!("Cached {} packages!", num),
        Err(e) => erm!("Failed to cache: {}", e),
    }
}

pub fn sync() {
    msg!("Syncing rid-meta...");
    bootstrap::get_rid_meta(false);
}

pub fn overwrite() {
    msg!("Overwrite-syncing rid-meta...");
    bootstrap::get_rid_meta(true);
}

fn display_list(mut plist: Vec<Package>) {
    plist.sort_by(|a, b| a.name.cmp(&b.name));

    for p in plist.iter() {
        let line = format!(
            "{}={} ~ {:?} {}",
            p.name, p.version, p.status, p.installed_version
        );
        let formatted_line = misc::format_line(&line, 32);
        println!("  {}", formatted_line);
    }

    vpr!("Displayed {} packages", plist.len())
}

pub fn list(pkgs: Vec<String>) {
    let pkgs = if pkgs.is_empty() {
        vec!["@all".to_string()]
    } else {
        pkgs
    };

    let pkgs = handle_sets(pkgs);

    // read_pkgs_json is far more performant than calling defp for pkg in pkgs
    let all_pkgs = read_pkgs_json().unwrap_or_else(|e| die!("Failed to read $RIDPKGSJSON: {}", e));
    let displayed: Vec<_> = pkgs
        .iter()
        .filter_map(|pkg| all_pkgs.iter().find(|p| p.name == *pkg).cloned())
        .collect();

    if displayed.len() != pkgs.len() {
        if let Some(pkg) = pkgs
            .iter()
            .find(|pkg| !displayed.iter().any(|p| p.name == **pkg))
        {
            die!("Package '{}' not found in $RIDPKGSJSON", pkg);
        }
    }

    msg!("PACKAGES");
    display_list(displayed);
}

fn dedup(mut vec: Vec<Package>) -> Vec<Package> {
    vec.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    vec.dedup_by(|a, b| a.name == b.name);
    vec
}

fn find_dependants(pkg: &str, pkg_list: &[Package]) -> Vec<Package> {
    pkg_list
        .iter()
        .filter(|p| p.deps.contains(&pkg.to_string()))
        .inspect(|p| vpr!("Found dependant package: '{}'", p.name))
        .cloned()
        .collect()
}

fn confirm_removal(pkg: &str, pkg_list: &[Package]) -> bool {
    vpr!("Checking dependants for '{}'", pkg);

    let mut dependants = find_dependants(pkg, pkg_list);
    dependants.retain(|p| p.name != *pkg);
    dependants = dedup(dependants);
    let len = dependants.len();

    vpr!("Found {} dependants", len);
    if dependants.is_empty() {
        return true;
    }

    erm!("Found {} dependant packages:", len);
    display_list(dependants);

    let message = format!("Remove '{}' ({} dependants)?", pkg, len);
    yn!(&message, false)
}

fn wrap_remove(pkg: &str, pkg_list: &mut Vec<Package>) {
    let p = defp(pkg);
    msg!("Removing {}-{}", p.name, p.version);

    eval_action('r', &p);
    match tracking::remove_package(pkg_list, pkg) {
        Ok(_) => msg!("Removed {}", pkg),
        Err(e) => erm!("Failed to track package removal '{}': {}", pkg, e),
    }

    clean::remove_tarballs(pkg);
}

pub fn remove(pkgs: Vec<String>, pkg_list: &mut Vec<Package>, force: bool) {
    for pkg in handle_sets(pkgs).iter() {
        if !force && !confirm_removal(pkg, pkg_list) {
            vpr!("Aborting removal for '{}'", pkg);
            return;
        }

        wrap_remove(pkg, pkg_list);
    }
}

fn gather_dependants(deps: &[String], pkg_list: &[Package]) -> Vec<Package> {
    let mut dependants = Vec::new();
    for dep in deps {
        let mut dep_dependants = find_dependants(dep, pkg_list);
        dep_dependants.retain(|p| p.name != *dep);
        dependants.extend(dep_dependants);
    }
    dedup(dependants)
}

pub fn remove_with_dependencies(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    // recursively removes a package and all its dependencies
    // this can be very dangerous if removal instructions are implemented for core packages
    let force = *FORCE.lock().unwrap();
    let pkgs = handle_sets(pkgs);

    for pkg in pkgs.iter() {
        if force {
            erm!("WARNING: Skipping all checks for dependants of '{}'", pkg)
        } else {
            vpr!("Checking for dependants of '{}' and its dependencies", pkg)
        }

        let pkg = defp(pkg);
        let deps = resolve_deps(&pkg);
        display_deps(&deps, pkg.clone());

        if !force {
            let dependants = gather_dependants(&deps, pkg_list);
            let len = dependants.len();
            if len != 0 {
                erm!("Found {} dependant packages:", len);
                display_list(dependants);
                let message = format!(
                    "Remove '{}' and its dependencies ({} total dependants)?",
                    pkg.name, len
                );
                if !yn!(&message, false) {
                    vpr!("Aborting removal since 'n' was selected");
                    return;
                }
            }
        }

        remove(deps, pkg_list, true);
    }
}

const TEMPLATE: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {pos}/{len} ({eta})";

pub fn prune(pkgs: Vec<String>) {
    let pkgs = handle_sets(pkgs);
    let mut tarballs_removed = 0;

    let length = pkgs.len() as u64;
    let bar = ProgressBar::new(length);

    bar.set_message("Pruning packages");
    bar.set_style(
        ProgressStyle::with_template(TEMPLATE)
            .unwrap()
            .progress_chars("#|-"),
    );
    bar.set_length(length);

    for pkg in pkgs {
        vpr!("Pruning {}", pkg);

        let p = defp(&pkg);
        let num_removed = clean::prune_sources(&p);
        vpr!("Pruned {} tarballs for {}", num_removed, pkg);
        tarballs_removed += num_removed;
        bar.inc(1);
    }

    let message = format!(
        "Pruned {} tarballs for {} packages",
        tarballs_removed, length
    );
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
                Err(e) => {
                    erm!("{}", e);
                    return;
                }
            };

            if do_install(&d, "") {
                wrap_install(d, pkg_list)
            }
        }
    }
}

fn update_package(p: Package, pkg_list: &mut Vec<Package>, force: bool) {
    if p.installed_version == p.version && !force {
        msg!("Package '{}' up to date", p.name);
        return;
    }
    msg!("Updating {}", p.name);

    fetch::fetch(&p);
    eval_action('u', &p);

    match tracking::add_package(pkg_list, &p) {
        Ok(_) => msg!("Updated to {}-{}", p.name, p.version),
        Err(e) => erm!("Failed to track package '{}': {}", p.name, e),
    }
}

pub fn update(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    let force = *FORCE.lock().unwrap();

    handle_sets(pkgs).iter().for_each(|pkg| {
        let p = defp(pkg);
        update_package(p, pkg_list, force);
    })
}

pub fn update_with_dependencies(pkgs: Vec<String>, pkg_list: &mut Vec<Package>) {
    let force = *FORCE.lock().unwrap();

    handle_sets(pkgs).iter().for_each(|pkg| {
        let p = defp(pkg);
        let deps = resolve_deps(&p);
        display_deps(&deps, p);

        deps.iter().for_each(|dep| {
            let d = defp(dep);
            update_package(d, pkg_list, force);
        })
    })
}

pub fn dependencies(pkgs: Vec<String>) {
    for pkg in handle_sets(pkgs).iter() {
        let p = defp(pkg);
        let d = resolve_deps(&p);
        display_deps(&d, p);
    }
}

pub fn dependants(pkgs: Vec<String>, pkg_list: &[Package]) {
    handle_sets(pkgs).iter().for_each(|pkg| {
        let mut dependants = find_dependants(pkg, pkg_list);
        dependants.retain(|p| p.name != *pkg);

        if dependants.is_empty() {
            msg!("No dependants for {}", pkg);
        } else {
            msg!("Dependants for {}", pkg);
            display_list(dependants);
        }
    });
}

pub fn news(pkgs: Vec<String>) {
    handle_sets(pkgs).iter().for_each(|pkg| {
        let p = defp(pkg);
        vpr!("Checking for news for package '{}'", p.name);
        vpr!("News: {}", p.news);

        if !p.news.is_empty() {
            msg!("News for {}:", p.name);
            pr!("\x1b[31;3m{}\x1b[0m\n", p.news);
        }
    })
}

pub fn get_files(pkgs: Vec<String>, force: bool) {
    for pkg in handle_sets(pkgs) {
        let p = defp(&pkg);
        let mut num_tarballs = 0;
        let mut num_extra = 0;

        if let Err(e) = fetch::download(&p, force) {
            match &*e.to_string() {
                "no link" | "extant" => {}
                _ => {
                    die!(
                        "Failed to fetch tarball for '{}' from '{}': {}",
                        p.name,
                        p.link,
                        e
                    )
                }
            }
        } else {
            num_tarballs += 1
        }

        for url in p.downloads {
            if let Err(e) = fetch::down(&url, force) {
                match &*e.to_string() {
                    "extant" => {}
                    _ => {
                        die!(
                            "Failed to fetch extra download for '{}' from '{}': {}",
                            p.name,
                            url,
                            e
                        );
                    }
                }
            } else {
                num_extra += 1
            }
        }

        msg!(
            "Downloaded {} tarballs and {} extra files",
            num_tarballs,
            num_extra
        );
    }
}

pub fn check_upstream() {
    if let Err(e) = exec("stab") {
        die!("Failed to check upstream: {}", e)
    }
}

pub fn validate_links() {
    if let Err(e) = exec("linkval") {
        die!("Failed to validate links: {}", e)
    }
}
