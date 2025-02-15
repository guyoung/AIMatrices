#[cfg(target_os="windows")]
pub use rocksdb_win::*;

#[cfg(not(target_os="windows"))]
pub use rocksdb::*;