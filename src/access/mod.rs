//! Access to open sqlite3 database by filename.
//!
//! The `core` module requires explicit authority to access files and such,
//! following the principle of least authority.
//!
//! This module provides the privileged functions to create such authorities.
//!
//! *TODO: move `mod access` to its own crate so that linking to `sqlite3` doesn't
//! bring in this ambient authority.*
#![unstable]

use libc::c_int;
use std::ptr;

use super::SqliteResult;
use core::{Access, DatabaseConnection};
use ffi;

use access::flags::OpenFlags;

// submodule KLUDGE around missing_docs for bitflags!()
#[allow(missing_docs)]
pub mod flags;

/// Open a database by filename.
///
/// *TODO: test for "Note that sqlite3_open() can be used to either
/// open existing database files or to create and open new database
/// files."*
///
///
/// Refer to [Opening A New Database][open] regarding URI filenames.
///
/// [open]: http://www.sqlite.org/c3ref/open.html
#[stable]
pub fn open(filename: &str, flags: Option<OpenFlags>) -> SqliteResult<DatabaseConnection> {
    DatabaseConnection::new(
        ByFilename {
            filename: filename,
            flags: flags.unwrap_or_default()
        })
}

/// Access to a database by filename
pub struct ByFilename<'a> {
    /// Filename or sqlite3 style URI.
    pub filename: &'a str,
    /// Flags for additional control over the new database connection.
    pub flags: OpenFlags
}

impl<'a> Access for ByFilename<'a> {
    fn open(self, db: *mut *mut ffi::sqlite3) -> c_int {
        self.filename.with_c_str({
            |filename| unsafe { ffi::sqlite3_open_v2(filename, db, self.flags.bits(), ptr::null()) }
        })
    }
}



#[cfg(test)]
mod tests {
    use std::default::Default;
    use super::ByFilename;
    use core::DatabaseConnection;


    #[test]
    fn open_file_db() {
        DatabaseConnection::new(
            ByFilename {
                filename: "/tmp/db1", flags: Default::default()
            })
            .unwrap();
    }
}

// Local Variables:
// flycheck-rust-crate-root: "lib.rs"
// End:
