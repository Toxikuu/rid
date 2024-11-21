// pm.rs
//
// package manager struct

use crate::package::Package;
use crate::resolvedeps::resolve_deps;
use crate::{die, msg, erm};
use crate::tracking::cache_changes;
use crate::utils::display_list;

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
        display_list(displayed);
    }

    pub fn cache(&mut self) {
        msg!("Caching packages to json...");
        match cache_changes(&mut self.pkglist, true) {
            Ok(n) => msg!("Cached {} packages", n),
            Err(e) => erm!("Failed to cache: {}", e)
        }
    }

    pub fn dependencies(&self) {
        for pkg in self.pkgs.iter() {
            let d = resolve_deps(pkg, self.pkglist.clone());
            msg!("Dependencies for {}", pkg);
            display_list(d);
        }
    }
}
