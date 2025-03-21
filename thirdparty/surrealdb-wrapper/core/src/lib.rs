#[macro_use]
extern crate tracing;

#[macro_use]
mod mac;

mod cf;
#[doc(hidden)]
pub mod ctx;
mod doc;
mod exe;
mod fnc;
mod vs;

pub mod sql;

#[doc(hidden)]
pub mod cnf;
#[doc(hidden)]
pub mod dbs;
#[doc(hidden)]
pub mod env;
#[doc(hidden)]
pub mod err;
#[doc(hidden)]
pub mod fflags;
#[cfg(all(not(target_arch = "wasm32"), surrealdb_unstable))]
#[doc(hidden)]
pub mod gql;
#[doc(hidden)]
pub mod iam;
#[doc(hidden)]
pub mod idg;
#[doc(hidden)]
pub mod idx;
#[doc(hidden)]
pub mod key;
#[doc(hidden)]
pub mod kvs;
#[doc(hidden)]
pub mod options;
#[doc(hidden)]
pub mod rpc;
#[doc(hidden)]
pub mod syn;

#[cfg(feature = "ml")]
#[doc(hidden)]
pub use surrealml as ml;
#[cfg(feature = "ml")]
#[doc(hidden)]
pub mod obs;

#[doc(hidden)]
pub mod test_helpers {
    pub use crate::vs::conv::to_u128_be;
    pub use crate::vs::generate_versionstamp_sequences;
}

#[doc(hidden)]
/// Channels for receiving a SurrealQL database export
pub mod channel {
    pub use async_channel::bounded;
    pub use async_channel::unbounded;
    pub use async_channel::Receiver;
    pub use async_channel::Sender;
}
