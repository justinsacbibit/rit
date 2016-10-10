use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use clap::ArgMatches;
use rustc_serialize::hex::FromHex;

use lib::prelude::*;

pub fn cmd_cat_file(arg_matches: &ArgMatches) -> Result<()> {
    let mut oid = arg_matches.value_of("object").unwrap();

    // TODO: Require a flag argument

    let mut repository = try!(Repository::<FsRefDbBackend>::open::<&str>(None));
    let object = try!(repository.odb().read(&oid.from_hex().unwrap()));
    if arg_matches.is_present("pretty") {
        print!("{}", String::from_utf8_lossy(&object.content));
    } else if arg_matches.is_present("size") {
        println!("{}", object.content.len());
    } else if arg_matches.is_present("type") {
        println!("{}", object.otype);
    }

    Ok(())
}

