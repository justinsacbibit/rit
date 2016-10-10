use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path;
use std::result;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use rustc_serialize::hex::ToHex;

use super::super::error::Error;
use super::super::sha1::PackedSha1;
use super::super::types::Result;

use super::odb_backend::ObjectDbBackend;

pub struct LooseObjectDbBackend {
    objects_dir: path::PathBuf,
}

fn handle_object_dir_creation_err(err: io::Error) -> result::Result<(), io::Error> {
    if err.kind() == io::ErrorKind::AlreadyExists {
        Ok(())
    } else {
        Err(err)
    }
}

impl LooseObjectDbBackend {
    pub fn open<P: AsRef<path::Path>>(git_dir: P) -> Result<LooseObjectDbBackend> {
        let mut objects_dir = git_dir.as_ref().clone().to_path_buf();
        objects_dir.push("objects");
        try!(fs::create_dir(&objects_dir).or_else(handle_object_dir_creation_err));

        Ok(LooseObjectDbBackend {
            objects_dir: objects_dir,
        })
    }
}

impl ObjectDbBackend for LooseObjectDbBackend {
    fn read(&self, key: &[u8]) -> Result<Vec<u8>> {
        let hash = key.to_hex();
        let folder = &hash[0..2];
        let file = &hash[2..];

        let mut folder_path = self.objects_dir.clone();
        folder_path.push(folder);
        try!(fs::create_dir_all(&folder_path));

        let mut object_path = folder_path.clone();
        object_path.push(file);

        let mut file = try!(fs::File::open(&object_path));

        let mut decoder = ZlibDecoder::new(file);
        let mut full_content: Vec<u8> = Vec::new();
        try!(decoder.read_to_end(&mut full_content));

        Ok(full_content)
    }

    fn add(&mut self, key: PackedSha1, content: &[u8]) -> Result<()> {
        let hash = key.to_hex();
        let folder = &hash[0..2];
        let file = &hash[2..];

        let mut folder_path = self.objects_dir.clone();
        folder_path.push(folder);
        try!(fs::create_dir_all(&folder_path));

        let mut object_path = folder_path.clone();
        object_path.push(file);

        let mut file = try!(fs::File::create(&object_path));

        let mut e = ZlibEncoder::new(file, Compression::Best);
        try!(e.write(content));
        try!(e.finish());

        Ok(())
    }

    fn delete(&mut self, key: PackedSha1) -> Result<()> {
        unimplemented!()
    }
}

