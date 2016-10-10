use std::fmt;
use std::result;

use super::error::Error;

pub type Result<T> = result::Result<T, Error>;

pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
            ObjectType::Tag => write!(f, "tag"),
        }
    }
}

impl<'a> From<&'a [u8]> for ObjectType {
    fn from(type_str: &'a [u8]) -> ObjectType {
        if "blob".as_bytes() == type_str {
            ObjectType::Blob
        } else if "tree".as_bytes() == type_str {
            ObjectType::Tree
        } else if "commit".as_bytes() == type_str {
            ObjectType::Commit
        } else {
            ObjectType::Tag
        }
    }
}

pub struct Object {
    pub otype: ObjectType,
    pub content: Vec<u8>,
}

