//! The `database` module contains the main coordinating logic
//! of SwiftDB.
//!
//! A [`Database`] instance runs on the main thread and is
//! responsible for startup and configuration, as well as
//! listening for incoming connections. It starts the
//! [`Backend`] thread, and various [`Connection`] threads
//! for each incoming network connection.
//!
//! [`Backend`]: crate::backend::Backend
//! [`Connection`]: crate::frontend::Connection
#[allow(clippy::module_inception)]
mod database;
mod lifecycle_error;

pub use database::Database;
pub use lifecycle_error::LifecycleError;
