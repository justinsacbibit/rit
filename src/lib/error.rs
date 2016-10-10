use rustc_serialize::hex::FromHexError;

use std::env;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ObjectDb(String),
    Repository(String),
    InvalidSpec,
    InvalidRef(String),
    FromHex(FromHexError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::ObjectDb(ref message) => write!(f, "Object DB error: {}", message),
            Error::Repository(ref message) => write!(f, "Repository error: {}", message),
            Error::InvalidSpec => unimplemented!(),
            Error::InvalidRef(ref invalid_ref) => unimplemented!(),
            Error::FromHex(ref from_hex_error) => unimplemented!(),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::ObjectDb(ref message) => "An object database related error",
            Error::Repository(ref message) => "A repository related error",
            _ => unimplemented!(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<FromHexError> for Error {
    fn from(err: FromHexError) -> Error {
        Error::FromHex(err)
    }
}

