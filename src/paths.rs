// src/paths.rs

use project_root::get_project_root;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref PROOT: PathBuf = {
        get_project_root().expect("Could not find project root")
    };

    pub static ref PKGSTXT: PathBuf = PROOT.join("packages.txt");
    pub static ref META:    PathBuf = PROOT.join("meta");
    pub static ref UTILS:   PathBuf = PROOT.join("utils");
    pub static ref SOURCES: PathBuf = PROOT.join("sources");
}
