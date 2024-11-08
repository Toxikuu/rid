// src/resolvedeps.rs
//
// responsible for dependency resolution

use crate::package::{defp, Package};
use crate::vpr;
use std::collections::HashSet;

fn deep_deps(pkg: &Package, resolved: &mut HashSet<String>, order: &mut Vec<String>) {
    for dep in &pkg.deps {
        if !resolved.contains(dep) {
            resolved.insert(dep.clone());

            let d = defp(dep);
            deep_deps(&d, resolved, order);
        }
    }
    order.push(pkg.name.clone());
}

pub fn resolve_deps(pkg: &Package) -> Vec<String> {
    let mut resolved = HashSet::new();
    let mut order = Vec::new();
    deep_deps(pkg, &mut resolved, &mut order);

    vpr!("Resolved dependencies: {:?}", order);
    order
}
