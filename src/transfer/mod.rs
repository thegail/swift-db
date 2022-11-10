//! The `transfer` module implements JSON serialization for
//! [`Document`][crate::schema::Document]s.
//!
//! Uses [`BareDocument`][bare_document::BareDocument] as
//! an intermediary format, which implements [`serde::Serialize`]
//! and [`serde::Deserialize`].
mod bare_document;
mod document_deserialize;
mod document_serialize;
mod errors;

pub use errors::DeserializationError;
