use std::fs;
use std::io;
use std::path;
use std::result;

use lib::error::*;
use lib::lock::*;
use lib::reflog::*;
use lib::oid::*;
use lib::reference::*;
use lib::signature::*;
use lib::refdb::*;
use lib::sha1::*;
use lib::types::*;

pub struct FsRefDbBackend {
    refs_dir: path::PathBuf,
}

fn handle_refs_dir_creation_err(err: io::Error) -> result::Result<(), io::Error> {
    if err.kind() == io::ErrorKind::AlreadyExists {
        Ok(())
    } else {
        Err(err)
    }
}

impl FsRefDbBackend {
    pub fn open<P: AsRef<path::Path>>(git_dir: P) -> Result<FsRefDbBackend> {
        let mut refs_dir = git_dir.as_ref().clone().to_path_buf();
        refs_dir.push("refs");
        let mut heads_dir = refs_dir.clone();
        let mut tags_dir = refs_dir.clone();

        try!(fs::create_dir(&refs_dir).or_else(handle_refs_dir_creation_err));

        heads_dir.push("heads");
        try!(fs::create_dir(&heads_dir).or_else(handle_refs_dir_creation_err));

        tags_dir.push("tags");
        try!(fs::create_dir(&tags_dir).or_else(handle_refs_dir_creation_err));

        Ok(FsRefDbBackend {
            refs_dir: refs_dir,
        })
    }
}

impl RefDbBackend for FsRefDbBackend {
    fn exists(&self, ref_name: &str) -> Result<bool> {
        unimplemented!()
    }

    fn lookup(&self, ref_name: &str) -> Result<Reference> {
        unimplemented!()
    }

    //fn iter(&self, glob: &str) -> Result<Iterator<Item=Reference>> {
        //unimplemented!()
    //}

    fn write(&self, reference: Reference, force: bool, who: GitSignature, message: &str, old: GitOid, old_target: &str) -> Result<()> {
        unimplemented!()
    }

    fn rename(&self, old_name: &str, new_name: &str, force: bool, who: GitSignature, message: &str) -> Result<Reference> {
        unimplemented!()
    }

    fn del(&self, ref_name: &str, old_id: GitOid, old_target: &str) -> Result<()> {
        unimplemented!()
    }

    fn compress(&self) -> Result<()> {
        unimplemented!()
    }

    fn has_log(&self, ref_name: &str) -> Result<bool> {
        unimplemented!()
    }

    fn ensure_log(&self, ref_name: &str) -> Result<()> {
        unimplemented!()
    }

    fn reflog_read(&self, ref_name: &str) -> Result<GitReflog> {
        unimplemented!()
    }

    fn reflog_write(&self, reflog: &GitReflog) -> Result<()> {
        unimplemented!()
    }

    fn reflog_rename(&self, old_name: &str, new_name: &str) -> Result<()> {
        unimplemented!()
    }

    fn reflog_delete(&self, name: &str) -> Result<()> {
        unimplemented!()
    }

    fn lock(&self, ref_name: &str) -> Result<Lock> {
        unimplemented!()
    }

    fn unlock(&self, lock: Lock) -> Result<()> {
        unimplemented!()
    }
}

