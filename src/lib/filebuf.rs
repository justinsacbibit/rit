use std::path;

use lib::types::Result;

pub struct GitFilebuf {
    path: path::PathBuf,
    lock_path: path::PathBuf,
}

impl GitFilebuf {
    pub fn write(buf: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

