// options.rs
//
// responsible for handling package options

use crate::vpr;

fn has_opts(pkg: &str) -> bool {
    pkg.contains("#")
}

pub fn split_opts(pkg: &str) -> (&str, Vec<String>) {
    if has_opts(pkg) {
        let opts: Vec<String> = pkg.split('#').skip(1).map(String::from).collect();
        vpr!("Found options: {:?} from '{}'", opts, pkg);
        return (pkg.split('#').next().unwrap(), opts)
    }
    (pkg, Vec::new())
}
