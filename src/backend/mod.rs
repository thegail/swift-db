//! The `backend` module performs disk operations for the database.
//!
//! Each [`Database`][crate::database::Database] instance creates
//! and owns one [`Backend`], which responds to requests from
//! the frontend via an MPSC channel.
#[allow(clippy::module_inception)]
mod backend;
mod operation_error;
mod query;
mod request;
mod selection;
#[cfg(test)]
mod tests;

pub use backend::Backend;
pub use operation_error::OperationError;
pub use query::{Condition, Expression, Query};
pub use request::{Operation, Request, Response};
pub use selection::Selection;
