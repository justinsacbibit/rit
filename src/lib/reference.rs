use regex::Regex;

use lib::error::*;
use lib::oid::*;
use lib::types::*;

pub enum Reference {
    Symbolic(String),
    Oid(GitOid),
}

impl Reference {
    pub fn from_data(data: String) -> Result<Reference> {
        if data.starts_with("ref: refs/") {
            // symbolic
            unimplemented!()
        } else {
            let re = Regex::new(r"^[0-9a-f]{40}$").unwrap();
            // Test for sha1
            if !re.is_match(&data) {
                // TODO: Provide context in error?
                return Err(Error::InvalidRef(data));
            }

            let oid = try!(GitOid::from_str(&data));
            Ok(Reference::Oid(oid))
        }
    }

    //pub fn peel(&self, obj_type: ObjectType) -> Result<Object> {
        //unimplemented!()
    //}
}

