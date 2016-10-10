extern crate clap;
extern crate crypto;
extern crate flate2;
#[macro_use]
extern crate log;
extern crate regex;
extern crate rustc_serialize;
extern crate time;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::process;

use clap::{App, Arg, ArgMatches, SubCommand};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use log::LogLevelFilter;
use rustc_serialize::hex::ToHex;

mod commands;
//use commands::commit;
use commands::init;
//use commands::status;
use commands::hash_object;
use commands::cat_file;

mod lib;
use lib::error::Error;

mod logger;
use logger::Logger;

fn main() {
    let matches = App::new("Oxide")
        .arg(Arg::with_name("verbose")
             .help("turn on verbose logging")
             .short("v")
             )
        .subcommand(SubCommand::with_name("init")
                    .about("Creates a new oxide repository")
                    .version("0.1")
                    .author("Justin S.")
                    )
        .subcommand(SubCommand::with_name("status")
                    .about("Prints status of oxide repository")
                    .version("0.1")
                    .author("Justin S.")
                    )
        .subcommand(SubCommand::with_name("hash-object")
                    .about("Hashes an object")
                    .version("0.1")
                    .author("Justin S.")
                    .arg(Arg::with_name("file")
                         .help("the file whose contents to hash")
                         .index(1)
                         .conflicts_with("stdin")
                         .required(true)
                         )
                    .arg(Arg::with_name("stdin")
                         .help("reads content from standard input")
                         .long("stdin")
                         .conflicts_with("file")
                         )
                    .arg(Arg::with_name("write")
                         .help("writes object into object store")
                         .short("w")
                         .long("write")
                        )
                    )
        .subcommand(SubCommand::with_name("cat-file")
                    .about("Provide content or type and size information for repository objects")
                    .version("0.1")
                    .author("Justin S.")
                    .arg(Arg::with_name("object")
                         .help("The name of the object to show.")
                         .index(1)
                         .required(true)
                         )
                    .arg(Arg::with_name("type")
                         .help("show object type")
                         .short("t")
                         .long("type")
                         .conflicts_with("size")
                         )
                    .arg(Arg::with_name("size")
                         .help("show object size")
                         .short("s")
                         .long("size")
                         .conflicts_with("pretty")
                         )
                    .arg(Arg::with_name("pretty")
                         .help("pretty-print object's content")
                         .short("p")
                         .long("pretty")
                         .conflicts_with("type")
                         )
                    )
        .get_matches();

    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(Logger {
            verbose: matches.is_present("verbose"),
        })
    }).ok();

    let p1 = &init::execute;
    //let p2 = &status::execute;
    let p3 = &hash_object::cmd_hash_object;
    let p4 = &cat_file::cmd_cat_file;
    let mut map: HashMap<&str, &Fn(&ArgMatches) -> Result<(), Error>> = HashMap::new();
    map.insert("init", p1);
    //map.insert("status", p2);
    map.insert("hash-object", p3);
    map.insert("cat-file", p4);

    let command = match matches.subcommand_name() {
        Some(c) => map[c](matches.subcommand_matches(c).unwrap()),
        _ => { return; },
    };

    if let Err(e) = command {
        error!("{}", e);
        process::exit(1);
    }
}

