// upstream.rs
//
// responsible for checking upstream versions of packages

use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use ureq::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;
use std::time::Duration;
use std::thread::sleep;
use crate::{die, vpr, pr, erm};
use crate::package::Package;

fn extract_version(text: &str, pkg_str: &str) -> Result<String, String> {
    let version_pattern = Regex::new(
        r"\d+\.\d+\.\d+\.\d+(-rc\d+|-*a\d+|-*b\d+)?|\d+\.\d+\.\d+(-rc\d+|-*a\d+|-*b\d+)?|\d+\.\d+(-rc\d+|-*a\d+|-*b\d+)?|\d+(-rc\d+|-*a\d+|-*b*\d*)?"
    )
    .map_err(|e| e.to_string())?;

    let mut vers = text.to_lowercase();

    if vers.contains(pkg_str) {
        vers = vers.replace(pkg_str, "");
    }

    if vers.contains(".t") {
        vers = vers.split(".t").next().unwrap().to_string();
    }

    match version_pattern.find(&vers) {
        Some(m) => Ok(m.as_str().to_string()),
        _ => Err("Version not found".to_string()),
    }
}

fn determine_default_selector(url: &str) -> Option<&str> {
    let mut selectors = HashMap::new();
    selectors.insert(r"(?i).*github\.com.+\/tags", "div.Box-row:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > h2:nth-child(1) > a:nth-child(1)");
    selectors.insert(r"(?i).*github\.com.+\/releases\/latest", ".css-truncate > span:nth-child(2)");
    selectors.insert(r"(?i).*gitlab.+\/-\/tags", "li.gl-justify-between:nth-child(1) > div:nth-child(1) > a:nth-child(2)");
    selectors.insert(r"(?i).*pypi\.org.+", ".package-header__name");
    selectors.insert(r"(?i).*download\.savannah\..*gnu.org\/releases.+\/\?C=M&O=D", "tr.e:nth-child(2) > td:nth-child(1) > a:nth-child(1)");
    selectors.insert(r"(?i).*ftp\.gnu\.org\/.+\/\?C=M;O=D", "body > table:nth-child(2) > tbody:nth-child(1) > tr:nth-child(4) > td:nth-child(2) > a:nth-child(1)");
    selectors.insert(r"(?i).*archlinux\.org\/packages\/.+", "#pkgdetails > h2:nth-child(1)");
    selectors.insert(r"(?i).*repology\.org\/project.*\/information.*", ".version-newest");
    selectors.insert(r"(?i).*sourceforge\.net.+\/files.*", ".sub-label");
    selectors.insert(r"(?i).*freedesktop\.org\/.*releases\/.+\/\?C=M;O=D", "body > table:nth-child(2) > tbody:nth-child(1) > tr:nth-child(4) > td:nth-child(2) > a:nth-child(1)");

    let patterns: Vec<(Regex, &str)> = selectors.iter()
        .filter_map(|(key, selector)| Regex::new(key).ok().map(|regex| (regex, *selector)))
        .collect();

    for (pattern, selector) in patterns.iter() {
        if pattern.is_match(url) {
            return Some(selector);
        }
    }

    None
}

fn latest(pkg: &Package) -> Result<String, Box<dyn Error>> {
    if pkg.upstream.is_empty() {
        return Err("Empty upstream".into());
    }

    let mut attempt = 0;
    const MAX_ATTEMPTS: usize = 7;

    while attempt < MAX_ATTEMPTS {
        attempt += 1;

        let response = get(&pkg.upstream)
            .set("User-Agent", "rid")
            .call();

        match response {
            Ok(r) => {
                let document = Html::parse_document(&r.into_string()?);

                let default_selector = determine_default_selector(&pkg.upstream);

                let mut selector_str = pkg.selector.clone();
                if pkg.selector.is_empty() {
                    selector_str = default_selector.unwrap_or_else(|| die!("No valid selector found for '{}'", pkg)).to_string();
                }

                let selector = Selector::parse(&selector_str).map_err(|_| "Invalid selector pattern")?;

                if let Some(element) = document.select(&selector).next() {
                    if let Some(version_text) = element.text().next() {
                        match extract_version(version_text, &pkg.name) {
                            Ok(version) => {
                                return Ok(version);
                            },
                            Err(e) => {
                                erm!("Regex failed: {}", e);
                                erm!("Raw version text: {}", version_text);
                            }
                        }
                    }
                }
                vpr!("[ERROR] ({}/{}) Retrying '{}'", attempt, MAX_ATTEMPTS, pkg.name);
            }
            Err(e) => vpr!("[ERROR] ({}/{}) HTTP error for '{}': {}", attempt, MAX_ATTEMPTS, pkg.name, e),
        }

        sleep(Duration::from_millis(1337));
    }

    Err("Failed to fetch upstream version".into())
}

pub fn check_upstream(pkglist: &Vec<Package>) {
    // checks upstream versions (with aggressive parallelization)
    let mut num_threads: usize = 512;
    if pkglist.len() < 512 {
        num_threads = pkglist.len();
    }
    vpr!("Determined number of threads for check_upstream(): {}", num_threads);

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .stack_size(512 * 1024)
        .build()
        .unwrap();

    pool.install(|| {
        pkglist.par_iter().for_each(|pkg| {
            match latest(pkg) {
                Ok(version) => {
                    if version != *pkg.version {
                        let displayed_version = format!("\x1b[31;1m{}\x1b[0m", version);
                        pr!("{}: {} <-> {}", pkg.name, pkg.version, displayed_version);
                    } else {
                        vpr!("{}: {} <-> {}", pkg.name, pkg.version, version);
                    }
                }
                Err(e) => if e.to_string() != "Empty upstream" {
                    erm!("Error for '{}': {}", pkg.name, e);
                }
            }
        });
    });
}
