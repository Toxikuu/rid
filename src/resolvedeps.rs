// src/resolvedeps.rs
//
// responsible for dependency resolution

use crate::package::Package;
use crate::vpr;
use std::collections::HashSet;

fn deep_deps(pkg: &Package, pkglist: &Vec<Package>, resolved: &mut HashSet<String>, order: &mut Vec<String>) {
    for dep in &pkg.deps {
        if !resolved.contains(dep) {
            resolved.insert(dep.clone());

            let d = Package::new(dep, pkglist.to_vec());
            deep_deps(&d, pkglist, resolved, order);
        }
    }
    order.push(pkg.name.clone());
}

pub fn resolve_deps(pkg: &Package, pkglist: Vec<Package>) -> Vec<Package> {
    let mut resolved = HashSet::new();
    let mut order = Vec::new();
    deep_deps(pkg, &pkglist, &mut resolved, &mut order);

    vpr!("Resolved dependencies: {:?}", order);
    let deps = order.iter().map(|d| Package::new(d, pkglist.clone())).collect::<Vec<Package>>();
    deps
}
