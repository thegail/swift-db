//! The `frontend` is responsible for managing client connections.
//!
//! A [`Connection`] runs on its own thread, invokes the
//! [`language`][crate::language] parser to build statements, then
//! executes them. The [`Transaction`][transaction::Transaction] helper
//! struct manages transaction state and helps in the execution of
//! statements. Requests are passed to the backend via an MPSC channel,
//! the response is serialized, and sent back over the network stream.
mod connection;
mod frontend_error;
mod transaction;

pub use connection::Connection;
