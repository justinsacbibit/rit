use std::path;

use lib::refdb::*;
use lib::reference::*;
use lib::types::*;

pub struct RefDb<B: RefDbBackend> {
    backend: B,
}

impl<B: RefDbBackend> RefDb<B> {
    pub fn new(backend: B) -> Result<RefDb<B>> {
        Ok(RefDb {
            backend: backend,
        })
    }

    pub fn open<P: AsRef<path::Path>>(git_dir: P) -> Result<RefDb<FsRefDbBackend>> {
        let fs_refdb_backend = try!(FsRefDbBackend::open(&git_dir));
        Ok(RefDb {
            backend: fs_refdb_backend,
        })
    }

    pub fn resolve(&self, reference: Reference) -> Result<Reference> {
        match reference {
            Reference::Oid(_) => Ok(reference),
            Reference::Symbolic(ref sym_ref) => {
                self.backend.lookup(sym_ref)
            }
        }
    }
}

