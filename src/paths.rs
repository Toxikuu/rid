// src/paths.rs
//
// path constants

use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

lazy_static! {
    pub static ref RIDHOME: PathBuf = PathBuf::from(env::var("RIDHOME").expect("Set $RIDHOME"));
    pub static ref PKGSJSON: PathBuf =
        PathBuf::from(env::var("RIDPKGSJSON").expect("Set $RIDPKGSJSON"));
    pub static ref META: PathBuf = PathBuf::from(env::var("RIDMETA").expect("Set $RIDMETA"));
    pub static ref RBIN: PathBuf = PathBuf::from(env::var("RIDBIN").expect("Set $RIDBIN"));
    pub static ref SOURCES: PathBuf =
        PathBuf::from(env::var("RIDSOURCES").expect("Set $RIDSOURCES"));
    pub static ref TMPRID: PathBuf = PathBuf::from(env::var("RIDTMP").expect("Set $RIDTMP"));
    pub static ref BUILDING: PathBuf =
        PathBuf::from(env::var("RIDBUILDING").expect("Set $RIDBUILDING"));
    pub static ref EXTRACTION: PathBuf =
        PathBuf::from(env::var("RIDEXTRACTION").expect("Set $RIDEXTRACTION"));
    pub static ref TRASH: PathBuf = PathBuf::from(env::var("RIDTRASH").expect("Set $RIDTRASH"));
    pub static ref DEST: PathBuf = PathBuf::from(env::var("RIDDEST").expect("Set $RIDDEST"));
}
