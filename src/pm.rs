// pm.rs
//
// package manager struct

use crate::package::Package;
use crate::resolve::{resolve_deps, find_dependants, deep_dependants};
use crate::{die, vpr, pr, yn, msg, erm};
use crate::tracking;
use crate::utils::{dedup, display_list, do_install};
use crate::core::{confirm_removal, mint, download, fetch, prune_sources};
use crate::flags::FORCE;
use crate::upstream::check_upstream;
use crate::linkval::validate;
use indicatif::{ProgressStyle, ProgressBar};

pub struct PM {
    pub pkgs: Vec<Package>,
    pub pkglist: Vec<Package>,
}

impl PM {
    pub fn new(pkgs: Vec<Package>, pkglist: Vec<Package>) -> Self {
        PM { pkgs, pkglist }
    }

    pub fn list(&self) {

        let displayed = if self.pkgs.is_empty() {
            self.pkglist.clone()
        } else {
            self.pkgs
                .iter()
                .filter_map(|pkg| self.pkglist.iter().find(|p| p.name == *pkg.name).cloned())
                .collect()
        };

        if displayed.len() != self.pkgs.len() {
            if let Some(pkg) = self.pkgs
                .iter()
                .find(|pkg| !displayed.iter().any(|p| p.name == *pkg.name))
            {
                die!("Package '{}' missing from pkglist", pkg)
            }
        }

        msg!("PACKAGES");
        display_list(&displayed);
    }

    pub fn dependencies(&self) {
        for pkg in self.pkgs.iter() {
            let d = resolve_deps(pkg, &self.pkglist);
            msg!("Dependencies for {}", pkg);
            display_list(&d);
        }
    }

    // I don't like how this uses the force flag, but it's the best solution I
    // can think of for adding deep_dependants() functionality
    pub fn dependants(&self) {
        for pkg in self.pkgs.iter() {

            if *FORCE.lock().unwrap() {
                let mut all_dependants: Vec<Package> = Vec::new();
                let deps = resolve_deps(pkg, &self.pkglist);
                for dep in deps.iter() {
                    let d = find_dependants(dep, self.pkglist.clone());
                    all_dependants.extend(d);
                }
                let all_dependants = dedup(all_dependants);
                msg!("Deep dependants for {}", pkg);
                display_list(&all_dependants);
                return
            }

            let d = find_dependants(pkg, self.pkglist.clone());
            msg!("Direct dependants for {}", pkg);
            display_list(&d);
            vpr!("Tip: Use -fD for deep dependants")
        }
    }

    pub fn get(&self) {
        for pkg in self.pkgs.clone() {
            msg!("Getting files for {}", pkg);
            download(pkg, true);
        }
    }

    pub fn install(&mut self) {
        for pkg in self.pkgs.iter() {
            if do_install(pkg) {
                fetch(pkg);
                mint('i', pkg);
                tracking::add(&mut self.pkglist, pkg);
                msg!("Installed '{}'", pkg);
            }
        }
    }

    pub fn install_with_dependencies(&mut self) {
        for pkg in self.pkgs.iter() {
            let deps = resolve_deps(pkg, &self.pkglist);
            msg!("Dependencies for '{}'", pkg);
            display_list(&deps);
            for dep in deps.iter() {
                if do_install(dep) {
                    fetch(dep);
                    mint('i', dep);
                    tracking::add(&mut self.pkglist, pkg);
                    msg!("Installed '{}'", pkg);
                }
            }
        }
    }

    pub fn update(&mut self) {
        for pkg in self.pkgs.iter() {
            if pkg.installed_version == pkg.version && !*FORCE.lock().unwrap() {
                msg!("Package '{}' up to date", pkg);
                return;
            }

            msg!("Updating '{}'...", pkg);
            fetch(pkg);
            mint('u', pkg);

            tracking::add(&mut self.pkglist, pkg);
            msg!("Updated '{}'", pkg);
        }
    }

    pub fn update_with_dependencies(&mut self) {
        for pkg in self.pkgs.iter() {
            let deps = resolve_deps(pkg, &self.pkglist);
            msg!("Dependencies for '{}'", pkg);
            display_list(&deps);
            for dep in deps.iter() {
                if dep.installed_version == dep.version && !*FORCE.lock().unwrap() {
                    msg!("Package '{}' up to date", dep);
                    return;
                }

                msg!("Updating '{}'...", dep);
                fetch(dep);
                mint('u', dep);

                tracking::add(&mut self.pkglist, dep);
                msg!("Updated '{}'", dep);
            }
        }
    }

    pub fn remove(&mut self) {
        for pkg in self.pkgs.iter() {
            if !confirm_removal(pkg, &self.pkglist) {
                return
            }

            mint('r', pkg);
            tracking::rem(&mut self.pkglist, pkg);
            msg!("Removed '{}'", pkg);
        }
    }

    pub fn remove_with_dependencies(&mut self) {
        // recursively removes a package and all its dependencies
        // this can be very dangerous
        let force = *FORCE.lock().unwrap();
        for pkg in self.pkgs.iter() {
            if force {
                erm!("WARNING: Skipping all checks for deep dependants of '{}'", pkg)
            } else {
                vpr!("Checking for deep dependants of '{}'", pkg)
            }

            let deps = resolve_deps(pkg, &self.pkglist);
            msg!("Depencies for '{}'", pkg);
            display_list(&deps);

            if !force {
                let dependants = deep_dependants(&deps, &self.pkglist);
                let len = dependants.len();
                if len != 0 {
                    erm!("Found {} dependant packages:", len);
                    display_list(&dependants);
                    let message = format!(
                    "Remove '{}' and its dependencies ({} total dependants)?",
                    pkg.name, len);
                    if !yn!(&message, false) {
                        vpr!("Aborting removal since 'n' was selected");
                        return;
                    }
                }
            }

            for dep in deps.iter() {
                mint('r', dep);
                tracking::rem(&mut self.pkglist, dep);
                msg!("Removed '{}'", dep);
            }
        }
    }

    pub fn news(&mut self) {
        for pkg in self.pkgs.iter() {
            if !pkg.news.is_empty() {
                msg!("News for '{}':", pkg);
                pr!("\x1b[31;3m{}\x1b[0m\n", pkg.news);
            }
        }
    }

    pub fn prune(&self) {
        const BAR: &str = "{msg:.red} [{elapsed_precise}] [{wide_bar:.red/black}] {pos}/{len} ({eta})";
        let mut tarballs_removed = 0;
        let length = self.pkgs.len() as u64;
        let bar = ProgressBar::new(length);

        bar.set_message("Pruning packages");
        bar.set_style(
            ProgressStyle::with_template(BAR)
                .unwrap()
                .progress_chars("#|-"),
        );
        bar.set_length(length);

        for pkg in self.pkgs.iter() {
            vpr!("Pruning {}", pkg);
            let num_removed = prune_sources(pkg);
            vpr!("Pruned {} tarballs for '{}'", num_removed, pkg);
            tarballs_removed += num_removed;
            bar.inc(1);
        }

        bar.finish_with_message("Pruned");
        msg!("Pruned {} tarballs for {} packages", tarballs_removed, length);
    }

    pub fn check_upstream(&self) {
        let pkgs = if !self.pkgs.is_empty() {
            self.pkgs.clone()
        } else {
            self.pkglist.clone()
        };

        msg!("Checking upstream versions");
        check_upstream(&pkgs)
    }

    pub fn validate_links(&self) {
        let pkgs = if !self.pkgs.is_empty() {
            self.pkgs.clone()
        } else {
            self.pkglist.clone()
        };

        msg!("Validating links");
        validate(&pkgs)
    }
}
