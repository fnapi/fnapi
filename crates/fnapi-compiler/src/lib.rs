#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(never_type)]

use std::path::PathBuf;

use anyhow::Result;

mod file_compiler;
pub mod project;
pub mod target;
mod type_server;

/// One input file.
pub struct ServerApiFile {
    path: PathBuf,
}

impl ServerApiFile {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        Ok(Self { path })
    }
}
