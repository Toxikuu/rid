// src/paths.rs
//
// path constants

use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;
use crate::die;

fn wrap(var: &str) -> PathBuf {
    PathBuf::from(env::var(var).unwrap_or_else(|_| die!("Set ${}", var)))
}

lazy_static! {
    pub static ref RIDHOME:    PathBuf = wrap("RIDHOME");
    pub static ref PKGSJSON:   PathBuf = wrap("RIDPKGSJSON");
    pub static ref META:       PathBuf = wrap("RIDMETA");
    pub static ref SETS:       PathBuf = wrap("RIDSETS");
    pub static ref BIN:        PathBuf = wrap("RIDBIN");
    pub static ref SOURCES:    PathBuf = wrap("RIDSOURCES");
    pub static ref TMPRID:     PathBuf = wrap("RIDTMP");
    pub static ref BUILDING:   PathBuf = wrap("RIDBUILDING");
    pub static ref EXTRACTION: PathBuf = wrap("RIDEXTRACTION");
    pub static ref TRASH:      PathBuf = wrap("RIDTRASH");
    pub static ref DEST:       PathBuf = wrap("RIDDEST");
    pub static ref FAILED:     PathBuf = wrap("RIDFAILED");
}
