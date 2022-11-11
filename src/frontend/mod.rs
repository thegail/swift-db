//! The `frontend` is responsible for managing client connections.
//!
//! A [`Connection`] runs on its own thread, executing statements from
//! the client. The [`Transaction`] helper struct manages transaction
//! state and helps in the execution of statements.
//!
//! [`Transaction`]: transaction::Transaction
mod connection;
mod frontend_error;
mod transaction;

pub use connection::Connection;
