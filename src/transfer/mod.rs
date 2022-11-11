//! The `transfer` module implements JSON serialization for
//! [`Document`]s.
//!
//! Uses [`BareDocument`] as an intermediary format, which
//! implements [`serde::Serialize`] and [`serde::Deserialize`].
//!
//! [`Document`]: crate::schema::Document
//! [`BareDocument`]: bare_document::BareDocument
mod bare_document;
mod document_deserialize;
mod document_serialize;
mod errors;

pub use errors::DeserializationError;
