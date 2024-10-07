// src/paths.rs

use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref BUILD:   PathBuf   = PathBuf::from("/tmp/rid/building");
    pub static ref PKGSTXT: PathBuf = PathBuf::from("/etc/rid/packages.txt");
    pub static ref META:    PathBuf = PathBuf::from("/etc/rid/meta");
    pub static ref RBIN:    PathBuf = PathBuf::from("/etc/rid/rbin");
    pub static ref SOURCES: PathBuf = PathBuf::from("/etc/rid/sources");
}
