use std::path;

use super::super::sha1::{PackedSha1, hash};
use super::super::types::{Object, ObjectType};
use super::super::types::Result;

use super::odb_backend::ObjectDbBackend;
use super::loose_odb_backend::LooseObjectDbBackend;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use clap::ArgMatches;
use rustc_serialize::hex::ToHex;

pub struct ObjectDb {
    backends: Vec<Box<ObjectDbBackend>>,
}

impl ObjectDb {
    pub fn new(backends: Vec<Box<ObjectDbBackend>>) -> Result<ObjectDb> {
        Ok(ObjectDb {
            backends: backends,
        })
    }

    pub fn open<P: AsRef<path::Path>>(git_dir: P) -> Result<ObjectDb> {
        let loose_odb_backend = try!(LooseObjectDbBackend::open(&git_dir));
        Ok(ObjectDb {
            backends: vec![Box::new(loose_odb_backend)],
        })
    }

    pub fn read(&self, key: &[u8]) -> Result<Object> {
        let ref backend = self.backends[0];
        let full_content = try!(backend.read(key));

        let mut iterator = full_content.split(|num| *num == 0);

        let header = iterator.next().unwrap();
        let mut header_iterator = header.split(|num| *num == 0x20);
        let object_type = header_iterator.next().unwrap();
        //println!("{}", String::from_utf8_lossy(object_type));

        let content_length = header_iterator.next().unwrap();
        //println!("{}", String::from_utf8_lossy(content_length));

        let content_content = iterator.next().unwrap();
        //println!("{}", String::from_utf8_lossy(content_content));

        // TODO: parse trees
        Ok(Object {
            otype: ObjectType::Blob,
            content: content_content.to_vec(),
        })
    }

    pub fn write(&mut self, data: &[u8], object_type: ObjectType) -> Result<PackedSha1> {
        let full_content = prepend_header_to_content(data, object_type);
        let key = hash(&full_content);
        let ref mut backend = self.backends[0];
        try!(backend.add(key, &full_content));
        Ok(key)
    }

    pub fn hash(data: &[u8], object_type: ObjectType) -> Result<PackedSha1> {
        let full_content = prepend_header_to_content(data, object_type);
        let key = hash(&full_content);
        Ok(key)
    }
}

fn prepend_header_to_content(content: &[u8], object_type: ObjectType) -> Vec<u8> {
    let mut header: Vec<u8> = format!("{} {}\0", object_type, content.len()).bytes().collect();
    header.append(&mut content.to_vec());
    header
}

#[cfg(test)]
mod tests {
    use super::prepend_header_to_blob_content;

    #[test]
    fn it_works() {
        let content = "abc123".to_string();
        let mut content_as_bytes: Vec<u8> = content.into_bytes();

        let expected: Vec<u8> = "blob 6\0abc123".to_string().into_bytes();

        let output = prepend_header_to_blob_content(content_as_bytes);

        assert_eq!(output, expected);
    }
}

