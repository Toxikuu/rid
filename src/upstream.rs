// upstream.rs
//
// responsible for checking upstream versions of packages

use crate::cmd::static_exec;
use crate::config::CONFIG;
use crate::package::Package;
use crate::utils::remove_before_first_number as rbfn;
use crate::{vpr, pr, erm};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::error::Error;

fn vsort(versions: Vec<String>) -> Vec<String> {

    // TODO: Add config options for string exclusions
    let mut sorted_versions: Vec<String> = versions
        .into_iter()
        .filter(|line| !line.contains("^{}")    // exclude tag references
                    && !line.contains("rc")     // exclude release candidates
                    && !line.contains("alpha")  // exclude alphas
                    && !line.contains("beta"))  // exclude betas
        .map(|line| {
            line.strip_prefix('v').unwrap_or(&line).to_string();
            line.trim().to_string()
        })
        .collect();

    sorted_versions.sort_by(|a, b| {
        let parse_version = |v: &str| {

            // handle rc
            let v = if v.contains("rc") && !v.contains(".rc") {
                v.replace("rc", ".rc")
            } else {
                v.to_string()
            };

            let (stable_part, rc_part) = if v.contains(".rc") { // - is replaced with ., hence .rc
                let mut parts = v.splitn(2, ".rc");
                (parts.next().unwrap(), parts.next().unwrap_or("0").parse::<u32>().unwrap_or(0))
            } else {
                (v.as_str(), u32::MAX) // prioritize stable versions over rc
            };

            // versions without dots or with letters
            let stable_parts: Vec<u64> = stable_part
                .split('.')
                .map(|p| p.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
                .filter_map(|n| n.parse::<u64>().ok())
                .collect();

            let major = stable_parts.first().copied().unwrap_or(0); // .first() to satisfy rust-analyzer
            let minor = stable_parts.get(1).copied().unwrap_or(0);
            let patch = stable_parts.get(2).copied().unwrap_or(0);
            let build = stable_parts.get(3).copied().unwrap_or(0);

            (major, minor, patch, build, rc_part)
        };

        let a_parsed = parse_version(a);
        let b_parsed = parse_version(b);

        a_parsed.0.cmp(&b_parsed.0) // compare major
            .then_with(|| a_parsed.1.cmp(&b_parsed.1)) // minor
            .then_with(|| a_parsed.2.cmp(&b_parsed.2)) // patch
            .then_with(|| a_parsed.3.cmp(&b_parsed.3)) // build
            .then_with(|| a_parsed.4.cmp(&b_parsed.4)) // compare rc (stable comes first)
    });

    sorted_versions
}

fn latest(pkg: &Package) -> Result<String, Box<dyn Error>> {

    if pkg.upstream.is_empty() {
        return Err("Empty upstream".into());
    }

    if !pkg.version_command.is_empty() {
        for _ in 1..=CONFIG.upstream.retry_count {
            vpr!("Using custom version command");
            match static_exec(&pkg.version_command) {
                Ok(result) if !result.is_empty() => {
                    let v = result.trim();
                    let v = v.strip_suffix("^{}").unwrap_or(v);
                    let v = v.strip_prefix("v").unwrap_or(v);

                    return Ok(v.to_string());
                },
                Err(_) | Ok(_) => continue,
            }
        }
    }

    let mut output = String::new();
    let command = format!("git ls-remote --tags {}", pkg.upstream);
    for _ in 1..=CONFIG.upstream.retry_count {
        match static_exec(&command) {
            Ok(result) if !result.is_empty() => {
                output = result;
                break
            },
            Err(_) | Ok(_) => continue,
        }
    }

    if output.is_empty() {
        return Err("Failed to fetch version with default command".into())
    }

    let tags: Vec<&str> = output.lines().collect();

    let versions: Vec<String> = tags
        .iter()
        .filter_map(|tag| {
            if let Some(version) = tag.rsplit('/').next() {
                let version = version.to_lowercase()
                    .replace("_", "-")
                    .replace(&pkg.name, "")
                    .replace("-", ".");
                let version = rbfn(&version);
                let version = version.strip_suffix("^{}").unwrap_or(version);

                Some(version.to_string())
            } else { None }
        })
        .collect();

    let versions = vsort(versions);

    if let Some(latest_version) = versions.last() {
        Ok(latest_version.to_string())
    } else {
        Err("No versions found :(".into())
    }
}

pub fn check_upstream(pkglist: &Vec<Package>) {
    // checks upstream versions (with aggressive parallelization)

    let num_threads: usize = CONFIG.upstream.thread_count.min(pkglist.len());
    vpr!("Determined number of threads for check_upstream(): {}", num_threads);

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .stack_size(CONFIG.upstream.stack_size * 1024)
        .build()
        .unwrap();

    pool.install(|| {
        pkglist.par_iter().for_each(|pkg| {
            match latest(pkg) {
                Ok(version) => {
                    if version != *pkg.version {
                        let displayed_version = format!("\x1b[31;1m{}\x1b[0m", version);
                        erm!("\x1b[0;30;3m{}: {} <-> {}", pkg.name, pkg.version, displayed_version);
                    } else {
                        pr!("  {}: {} <-> {}", pkg.name, pkg.version, version);
                    }
                }
                Err(e) => if e.to_string() != "Empty upstream" {
                    erm!("Error for '{}': {}", pkg.name, e);
                }
            }
        });
    });
}
