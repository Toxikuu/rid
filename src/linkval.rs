// linkval.rs
//
// responsible for link validation

use crate::package::Package;
use crate::{erm, vpr, pr};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;

fn ping(url: &str, a: u8) -> Result<(), String> {
    if a > 3 {
        return Err("Invalid url".to_string())
    }

    vpr!("Pinging url '{}'", url);
    let response = ureq::head(url)
        .set("User-Agent", "rid")
        .call();

    if response.is_err() {
        vpr!("Retrying for '{}' (attempt #{})...", url, a + 1);
        ping(url, a + 1)?;
    }

    Ok(())
}

pub fn validate(validate_list: &[Package]) {
    let mut urls: Vec<String> = Vec::new();
    validate_list.iter().for_each(|p| urls.push(p.link.clone()));

    let mut extra_urls: Vec<String> = Vec::new();
    validate_list.iter().for_each(|p| extra_urls.extend(p.downloads.clone()));

    urls.extend(extra_urls);
    urls.retain(|s| !s.is_empty());

    let mut num_threads: usize = 32;
    if urls.len() < 32 {
        num_threads = urls.len();
    }
    vpr!("Determined number of threads for linkval: {}", num_threads);

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .stack_size(256 * 1024)
        .build()
        .unwrap();

    pool.install(|| {
        urls.par_iter().for_each(|url| {
            if let Err(e) = ping(url, 0) {
                erm!("Invalid url '{}': {}", url, e);
            } else {
                pr!("Valid url: {}", url)
            }
        });
    });
}
