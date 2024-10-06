// src/resolvedeps.rs

use std::collections::HashSet;
use crate::package::{form_package, Package};

use crate::pr;

fn deep_deps(pkg: &Package, resolved: &mut HashSet<String>, order: &mut Vec<String>) {
    for dep in &pkg.deps {
        if !resolved.contains(dep) {
            resolved.insert(dep.clone());

            match form_package(dep) {
                Ok(dep_pkg) => {
                    deep_deps(&dep_pkg, resolved, order);
                },
                Err(e) => {
                    eprintln!("Failed to load dependency '{}': {}", dep, e)
                },
            }
        }
    }
    order.push(pkg.name.clone());
}

pub fn resolve_deps(pkg: &Package) -> Vec<String> {
    let mut resolved = HashSet::new();
    let mut order = Vec::new();
    deep_deps(pkg, &mut resolved, &mut order);

    pr!(format!("Resolved dependencies: {:?}", order), 'v');
    order
}
