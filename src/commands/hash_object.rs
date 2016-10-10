use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use clap::ArgMatches;
use rustc_serialize::hex::ToHex;

use lib::prelude::*;

pub fn cmd_hash_object(arg_matches: &ArgMatches) -> Result<()> {
    let content = if arg_matches.is_present("stdin") {
        let mut buffer = Vec::new();
        try!(io::stdin().read_to_end(&mut buffer));
        buffer
    } else {
        let mut file = try!(File::open(arg_matches.value_of("file").unwrap()));
        let mut data: Vec<u8> = Vec::new();
        try!(file.read_to_end(&mut data));
        data
    };

    let hash = if arg_matches.is_present("write") {
        let mut repository = try!(Repository::<FsRefDbBackend>::open::<&str>(None));
        // TODO: Support other object types
        try!(repository.odb().write(&content, ObjectType::Blob))
    } else {
        try!(ObjectDb::hash(&content, ObjectType::Blob))
    };

    println!("{}", hash.to_hex());
    Ok(())
}

