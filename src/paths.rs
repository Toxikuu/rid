// src/paths.rs
//
// path constants

use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

lazy_static! {
    pub static ref RIDHOME: PathBuf =
        PathBuf::from(env::var("RIDHOME").unwrap_or_else(|_| "/etc/rid".to_string()));
    pub static ref PKGSJSON: PathBuf =
        PathBuf::from(env::var("PKGSJSON").unwrap_or_else(|_| "/etc/rid/pkgs.json".to_string()));
    pub static ref META: PathBuf =
        PathBuf::from(env::var("META").unwrap_or_else(|_| "/etc/rid/meta".to_string()));
    pub static ref RBIN: PathBuf =
        PathBuf::from(env::var("RBIN").unwrap_or_else(|_| "/etc/rid/rbin".to_string()));
    pub static ref SOURCES: PathBuf =
        PathBuf::from(env::var("SOURCES").unwrap_or_else(|_| "/etc/rid/sources".to_string()));
    pub static ref TMPRID: PathBuf =
        PathBuf::from(env::var("TMPRID").unwrap_or_else(|_| "/tmp/rid/".to_string()));
    pub static ref BUILDING: PathBuf =
        PathBuf::from(env::var("BUILDING").unwrap_or_else(|_| "/tmp/rid/building".to_string()));
    pub static ref EXTRACTION: PathBuf =
        PathBuf::from(env::var("EXTRACTION").unwrap_or_else(|_| "/tmp/rid/extraction".to_string()));
    pub static ref TRASH: PathBuf =
        PathBuf::from(env::var("TRASH").unwrap_or_else(|_| "/tmp/rid/trash".to_string()));
    pub static ref DEST: PathBuf =
        PathBuf::from(env::var("DEST").unwrap_or_else(|_| "/tmp/rid/dest".to_string()));
}
