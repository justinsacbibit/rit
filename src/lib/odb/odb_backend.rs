use super::super::error::Error;
use super::super::sha1::PackedSha1;
use super::super::types::Result;

pub trait ObjectDbBackend {
    fn read(&self, key: &[u8]) -> Result<Vec<u8>>;
    fn add(&mut self, key: PackedSha1, content: &[u8]) -> Result<()>;
    fn delete(&mut self, key: PackedSha1) -> Result<()>;
}

