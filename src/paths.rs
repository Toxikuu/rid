// src/paths.rs

use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref ETCRID: PathBuf = PathBuf::from("/etc/rid");
    pub static ref PKGSTXT: PathBuf = PathBuf::from("/etc/rid/packages.txt");
    pub static ref META: PathBuf = PathBuf::from("/etc/rid/meta");
    pub static ref RBIN: PathBuf = PathBuf::from("/etc/rid/rbin");
    pub static ref SOURCES: PathBuf = PathBuf::from("/etc/rid/sources");
    pub static ref TMPRID: PathBuf = PathBuf::from("/tmp/rid/");
    pub static ref BUILDING: PathBuf = PathBuf::from("/tmp/rid/building");
    pub static ref EXTRACTION: PathBuf = PathBuf::from("/tmp/rid/extraction");
    pub static ref TRASH: PathBuf = PathBuf::from("/tmp/rid/trash");
    pub static ref DEST: PathBuf = PathBuf::from("/tmp/rid/dest");
}
