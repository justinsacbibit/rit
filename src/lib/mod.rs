pub mod error;
pub mod filebuf;
pub mod filesystem;
mod lock;
pub mod odb;
pub mod oid;
pub mod refdb;
pub mod reference;
pub mod reflog;
pub mod repository;
pub mod sha1;
pub mod signature;
pub mod time;
pub mod tree;
pub mod types;

/// Basic data structures and traits used throughout `rit`.
pub mod prelude {
    pub use lib::error::*;
    pub use lib::filebuf::*;
    pub use lib::filesystem::*;
    pub use lib::odb::*;
    pub use lib::oid::*;
    pub use lib::refdb::*;
    pub use lib::reference::*;
    pub use lib::reflog::*;
    pub use lib::repository::*;
    pub use lib::sha1::*;
    pub use lib::signature::*;
    pub use lib::time::*;
    pub use lib::tree::*;
    pub use lib::types::*;
}

