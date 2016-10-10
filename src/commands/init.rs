use clap::ArgMatches;

use lib::prelude::*;

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let repository = try!(Repository::<FsRefDbBackend>::init::<&str>(None, false));
    println!("Initialized empty Git repository in {}/", repository.path.unwrap().display());

    Ok(())
}

