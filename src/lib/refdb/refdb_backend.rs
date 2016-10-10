use lib::lock::*;
use lib::oid::*;
use lib::reference::*;
use lib::reflog::*;
use lib::signature::*;
use lib::types::*;

// TODO: Consider moving some of these functions to Reference itself.
//
// One option would be to add explicit lifetimes (as References would have to
// hold a reference to the RefDb/RefDbBackend)
//
// Another option would be use reference counting (not really necessary; a
// refdb and its backend should live for as long as their repository, and a
// repository should be the longest living object in a program.

pub trait RefDbBackend {
    /// Queries the refdb backend to determine if the given ref_name exists.
    fn exists(&self, ref_name: &str) -> Result<bool>;

    /// Queries the refdb backend for a given reference.
    fn lookup(&self, ref_name: &str) -> Result<Reference>;

    /// Returns an iterator over references which match the glob pattern.
    // TODO
    //fn iter(&self, glob: &str) -> Result<Iterator<Item=Reference>>;

    /// Writes the given reference to the refdb.
    fn write(&self, reference: Reference, force: bool, who: GitSignature, message: &str, old: GitOid, old_target: &str) -> Result<()>;

    /// Renames the given reference in the refdb;
    fn rename(&self, old_name: &str, new_name: &str, force: bool, who: GitSignature, message: &str) -> Result<Reference>;

    /// Deletes the given reference (and if necessary its reflog) from the
    /// refdb.
    fn del(&self, ref_name: &str, old_id: GitOid, old_target: &str) -> Result<()>;

    /// Suggests that the refdb compress or optimize its references. This
    /// mechanism is implementation specific. (For on-disk reference
    /// databases, this may pack all loose references.) A refdb implementation
    /// is not required to do anything when this function is called.
    fn compress(&self) -> Result<()>;

    /// Query whether a particular reference has a log (may be empty).
    fn has_log(&self, ref_name: &str) -> Result<bool>;

    /// Make sure a particular reference will have a reflog which will be
    /// appended to on writes.
    fn ensure_log(&self, ref_name: &str) -> Result<()>;

    /// Read the reflog for the given reference name.
    fn reflog_read(&self, ref_name: &str) -> Result<GitReflog>;

    /// Write a reflog.
    fn reflog_write(&self, reflog: &GitReflog) -> Result<()>;

    /// Rename a reflog.
    fn reflog_rename(&self, old_name: &str, new_name: &str) -> Result<()>;

    /// Delete a reflog.
    fn reflog_delete(&self, name: &str) -> Result<()>;

    /// Lock a reference. The return value will be passed to the unlock
    /// function.
    fn lock(&self, ref_name: &str) -> Result<Lock>;

    /// Unlock a reference.
    fn unlock(&self, lock: Lock) -> Result<()>;
}

