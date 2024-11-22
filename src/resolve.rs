// src/resolve.rs
//
// responsible for dependency/dependant resolution

use crate::package::Package;
use crate::vpr;
use crate::utils::dedup;
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

pub fn find_dependants(pkg: Package, pkglist: Vec<Package>) -> Vec<Package> {
    let mut dependants = pkglist
        .iter()
        .filter(|p| p.deps.contains(&pkg.name))
        .inspect(|p| vpr!("Found dependant package: '{}'", p))
        .cloned()
        .collect::<Vec<Package>>();

    dependants.retain(|p| p.name != *pkg.name);
    dedup(dependants)
}

pub fn deep_dependants(deps: Vec<Package>, pkglist: &[Package]) -> Vec<Package> {
    let mut dependants = Vec::new();
    for dep in deps {
        let dep_dependants = find_dependants(dep, pkglist.to_vec());
        dependants.extend(dep_dependants);
    }
    dedup(dependants)
}
