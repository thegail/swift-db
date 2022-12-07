//! The `backend` module performs disk operations for the database.
//!
//! See [`Backend`].
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
pub use selection::Reference;
