use regex::Regex;

use std::env;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path;
use std::result;

use lib::error::*;
use lib::filesystem::*;
use lib::odb::*;
use lib::refdb::*;
use lib::reference::*;
use lib::tree::*;
use lib::types::*;

pub struct Repository<R: RefDbBackend> {
    odb: ObjectDb,
    pub path: Option<path::PathBuf>,
    refdb: RefDb<R>,
}

fn handle_git_dir_creation_err(err: io::Error) -> result::Result<(), io::Error> {
    if err.kind() == io::ErrorKind::AlreadyExists {
        // TODO: Re-init
        Ok(())
    } else {
        Err(err)
    }
}

impl<R: RefDbBackend> Repository<R> {
    pub fn odb(&mut self) -> &mut ObjectDb {
        &mut self.odb
    }

    pub fn discover<P: AsRef<path::Path>>(start_path: Option<P>) -> Result<path::PathBuf> {
        let mut current_path: path::PathBuf = match start_path {
            Some(path) => path.as_ref().to_path_buf(),
            None => {
                match env::current_dir() {
                    Ok(dir) => dir,
                    Err(e) => {
                        return Err(Error::Io(e));
                    }
                }
            }
        };

        let mut possible_path = current_path.clone();
        possible_path.push(".git");

        let original_path = possible_path.clone();

        loop {
            match Filesystem::entry(&possible_path) {
                Ok(Entry::Directory) => {
                    return Ok(possible_path);
                }

                Ok(Entry::DoesNotExist) => {}

                Err(e) => {
                    return Err(Error::Io(e));
                }

                _ => {
                    return Err(Error::Repository(format!("{} exists but it's not a directory", possible_path.display())));
                }
            }

            if !current_path.pop() {
                return Err(Error::Repository(format!("Not a git repository (or any of the parent directories): {}", original_path.display())));
            }
        }
    }

    pub fn open<P: AsRef<path::Path>>(maybe_git_dir: Option<P>) -> Result<Repository<FsRefDbBackend>> {
        let git_dir = match maybe_git_dir {
            Some(p) => p.as_ref().to_path_buf(),
            None => {
                let current_dir = try!(env::current_dir());
                match Repository::<R>::discover(Some(current_dir)) {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        };

        let object_db = try!(ObjectDb::open(&git_dir));
        let ref_db = try!(RefDb::<FsRefDbBackend>::open(&git_dir));

        Ok(Repository {
            odb: object_db,
            path: Some(git_dir),
            refdb: ref_db,
        })
    }

    pub fn init<P: AsRef<path::Path>>(maybe_repository_path: Option<P>, is_bare: bool) -> Result<Repository<FsRefDbBackend>> {
        if is_bare {
            // TODO
            unimplemented!();
        }

        let repository_path = match maybe_repository_path {
            Some(p) => p.as_ref().to_path_buf(),
            None => try!(env::current_dir()),
        };

        let mut git_dir = repository_path.clone();
        git_dir.push(".git");
        try!(fs::create_dir(&git_dir).or_else(handle_git_dir_creation_err));

        let mut head_path = git_dir.clone();
        head_path.push("HEAD");
        let mut head_file = try!(fs::File::create(&head_path));
        try!(head_file.write_all("ref: refs/heads/master".as_bytes()));

        Repository::<FsRefDbBackend>::open(Some(git_dir))
    }

    fn head_tree(&self) -> Result<GitTree> {
        let head = try!(self.head());

        match head {
            Reference::Symbolic(reference) => {
                // HEAD is (probably) pointing to a branch
                match self.refdb.resolve() {
                    Reference::Symbolic(_) => {
                        // wat
                        unimplemented!()
                    }

                    Reference::Oid(oid) => {
                        self.odb.
                    }
                }
            }

            Reference::Oid(oid) => {
                // HEAD is detached
                unimplemented!()
            }
        }
    }

    pub fn head(&self) -> Result<Reference> {
        self.lookup_ref("HEAD")
    }

    // Create a fake repository
    #[allow(dead_code)]
    pub fn wrap_odb_refdb(odb: ObjectDb, refdb: RefDb<R>) -> Repository<R> {
        Repository {
            odb: odb,
            path: None,
            refdb: refdb,
        }
    }

    // Reference
    pub fn lookup_ref(&self, name: &str) -> Result<Reference> {
        if name.starts_with("refs/") {
            // Use refdb
            unimplemented!()
        } else {
            if self.path.is_none() {
                // TODO: Return appropriate error
                unimplemented!();
            }

            // Must start and end with an uppercase letter
            // Can only contain uppercase letters or underscores
            // TODO: Only compile the regex once
            let re = Regex::new(r"^[A-Z][A-Z_]*[A-Z]$").unwrap();

            if !re.is_match(&name) {
                // TODO: Include additional error information?
                return Err(Error::InvalidSpec);
            }

            // Read from file and return reference
            // TODO: Refactor this logic into another function if necessary

            let mut path = self.path.as_ref().unwrap().clone();
            path.push(name);

            let mut file_content = String::new();
            let mut file = try!(fs::File::open(path));
            try!(file.read_to_string(&mut file_content));

            let reference = try!(Reference::from_data(file_content));
            Ok(reference)
        }
    }
}

