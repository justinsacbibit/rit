use rustc_serialize::hex::{FromHex, ToHex};

use std::cmp;

use lib::error::*;
use lib::types::*;

pub const GitOidRawSize: usize = 20;

pub type GitOidRaw = [u8; GitOidRawSize];

pub const GitOidHexSize: usize = GitOidRawSize * 2;

pub const GitOidMinPrefixLen: usize = 4;

/// Unique identity of any object (commit, tree, blob, or tag).
pub struct GitOid {
    /// Raw binary formatted id
    id: GitOidRaw,
}

impl GitOid {
    /// Parse a hex formatted object id into an GitOid.
    /// The input hex should be 40 characters or less.
    pub fn from_str(hex: &str) -> Result<GitOid> {
        let hex_as_bytes = try!(hex.from_hex());
        if hex_as_bytes.len() > GitOidRawSize {
            return Err(Error::InvalidSpec);
        }
        let mut id = [0u8; GitOidRawSize];
        for i in 0..GitOidRawSize {
            id[i] = hex_as_bytes[i];
        }

        Ok(GitOid {
            id: id,
        })
    }

    /// Copy an already raw oid into an GitOid.
    pub fn from_raw(id: GitOidRaw) -> GitOid {
        GitOid {
            id: id,
        }
    }

    /// Format an GitOid into a hex string.
    pub fn fmt(&self) -> String {
        self.id.to_hex()
    }

    /// Format an GitOid into a partial hex string.
    /// The `len` parameter determines the number of characters in the string.
    pub fn partial_fmt(&self, len: usize) -> String {
        let partial = &self.id[..len];
        partial.to_hex()
    }

    /// Format an GitOid into a loose-object path string.
    /// The resulting string is "aa/...", where "aa" is the first two hex
    /// digits of the GitOid and "..." is the remaining 38 digits.
    pub fn path_fmt(&self) -> String {
        let folder = self.id[..1].to_hex();
        let file = self.id[1..].to_hex();
        format!("{}/{}", folder, file)
    }
}

impl PartialEq for GitOid {
    fn eq(&self, other: &GitOid) -> bool {
        self.id == other.id
    }
}

impl Eq for GitOid {}

impl Ord for GitOid {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for GitOid {
    fn partial_cmp(&self, other: &GitOid) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

