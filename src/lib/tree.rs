use lib::oid::*;

pub struct GitTreeEntry {
    filename: String,
    oid: GitOid,
}

pub struct GitTree {
    entries: Vec<GitTreeEntry>,
}

