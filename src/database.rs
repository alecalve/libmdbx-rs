use libc::c_uint;
use std::ffi::CString;
use std::ptr;

use ffi;

use error::{LmdbResult, lmdb_result};

/// A handle to an individual database in an environment.
///
/// A database handle denotes the name and parameters of a database in an environment.
#[derive(Show, Clone, Copy, Eq, PartialEq)]
pub struct Database {
    dbi: ffi::MDB_dbi,
}

impl Database {

    /// Opens a new database handle in the given transaction.
    ///
    /// Prefer using `Environment::open_db`, `Environment::create_db`, `TransactionExt::open_db`,
    /// or `RwTransaction::create_db`.
    #[doc(hidden)]
    pub unsafe fn new(txn: *mut ffi::MDB_txn,
                      name: Option<&str>,
                      flags: c_uint)
                      -> LmdbResult<Database> {
        let c_name = name.map(|n| CString::from_slice(n.as_bytes()));
        let name_ptr = if let Some(ref c_name) = c_name { c_name.as_ptr() } else { ptr::null() };
        let mut dbi: ffi::MDB_dbi = 0;
        try!(lmdb_result(ffi::mdb_dbi_open(txn, name_ptr, flags, &mut dbi)));
        Ok(Database { dbi: dbi })
    }

    /// Returns the underlying LMDB database handle.
    ///
    /// The caller **must** ensure that the handle is not used after the lifetime of the
    /// environment, or after the database has been closed.
    pub fn dbi(&self) -> ffi::MDB_dbi {
        self.dbi
    }
}
