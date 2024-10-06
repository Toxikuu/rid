// src/paths.rs

use project_root::get_project_root;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref PROOT: PathBuf = {
        get_project_root().expect("Could not find project root")
    };

    pub static ref PKGSTXT: PathBuf = PathBuf::from("/etc/rid/packages.txt");
    pub static ref META:    PathBuf = PathBuf::from("/etc/rid/meta");
    pub static ref UTILS:   PathBuf = PathBuf::from("/etc/rid/utils");
    pub static ref SOURCES: PathBuf = PathBuf::from("/etc/rid/sources");
}
