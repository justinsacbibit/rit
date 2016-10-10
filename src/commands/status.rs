extern crate regex;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path;

use clap::ArgMatches;

use lib::prelude::*;

fn get_branch(head_content: &str) -> &str {
    let re = regex::Regex::new(r"^ref: refs/heads/([\w-_]+)\n$").unwrap();
    let captures = re.captures(&head_content).unwrap();
    let branch = captures.at(1).unwrap();
    branch
}

fn status(ox_dir: path::PathBuf) -> Result<(), String> {
    let mut head_path = ox_dir.clone();
    head_path.push("HEAD");

    let mut head_file = File::open(&head_path).unwrap();
    let mut head_content = String::new();
    head_file.read_to_string(&mut head_content);

    let branch = get_branch(&head_content);
    println!("On branch {}", branch);

    Ok(())
}

pub fn execute(matches: &ArgMatches) -> Result<(), String> {
    let mut current_dir = match env::current_dir() {
        Ok(val) => val,
        Err(e) => {
            let message = format!("An error occurred when trying to get the current directory: {}", e);
            return Err(message);
        }
    };

    let mut immediate_ox_dir = current_dir.clone();
    immediate_ox_dir.push(".oxide");

    loop {
        let mut ox_dir = current_dir.clone();
        ox_dir.push(".oxide");

        match filesystem::entry(&ox_dir) {
            Ok(Entry::Directory) => {
                return status(ox_dir);
            }

            Ok(Entry::DoesNotExist) => {},

            Err(e) => {
                let message = format!("An error occurred checking for a {} directory: {}", ox_dir.display(), e);
                return Err(message);
            }

            _ => {
                return Err(format!("{} exists but it's not a directory", ox_dir.display()));
            }
        }
        let parent_dir = current_dir.clone();

        if !current_dir.pop() {
            return Err(format!("Not an oxide repository (or any of the parent directories): {}", immediate_ox_dir.display()));
        }
    }
}

