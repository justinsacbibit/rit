use std::fmt;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path;

#[derive(Debug)]
pub enum Entry {
    File,
    Executable,
    Directory,
    DoesNotExist,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Entry::File => write!(f, "File"),
            Entry::Executable => write!(f, "Executable"),
            Entry::Directory => write!(f, "Directory"),
            Entry::DoesNotExist => write!(f, "Does not exist"),
        }
    }
}

fn entry_from_metadata(metadata: fs::Metadata) -> Entry {
    if metadata.is_dir() {
        return Entry::Directory;
    }

    let mode = metadata.permissions().mode();
    let entry = match mode & 0o111 {
        0 => Entry::File,
        _ => Entry::Executable,
    };
    entry
}

fn entry_from_error(err: io::Error) -> Result<Entry, io::Error> {
    if let Some(2) = err.raw_os_error() {
        Ok(Entry::DoesNotExist)
    } else {
        Err(err)
    }
}

pub struct Filesystem;

impl Filesystem {
    pub fn entry<P: AsRef<path::Path>>(path: P) -> Result<Entry, io::Error> {
        fs::metadata(path).map(entry_from_metadata).or_else(entry_from_error)
    }
}

