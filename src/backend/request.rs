use crate::backend::{OperationError, Query, Selection};
use crate::schema::Document;
use std::sync::mpsc::Sender;

/// A request for the [`Backend`][crate::backend::Backend] to execute
/// some [`Operation`].
pub struct Request {
    /// The [`Operation`] to execute.
    pub operation: Operation,
    /// A sender to send the [`Response`] back to the frontend.
    pub return_channel: Sender<Result<Response, OperationError>>,
}

/// An operation for the [`Backend`][crate::backend::Backend] to
/// execute.
pub enum Operation {
    /// Find one [`Document`] in a collection.
    ///
    /// See [`Query`]. Returns a [`Response::Selection`].
    FindOne { query: Query },
    /// Create a [`Document`] on a collection.
    ///
    /// Returns a [`Response::Selection`].
    Create { document: Document },
    /// Read some fields of the [`Document`] referred to by
    /// `selection`.
    ///
    /// Takes a list of the field IDs to read. Returns a
    /// [`Response::Document`].
    Read {
        selection: Selection,
        fields: Vec<u16>,
    },
}

/// A response to a [`Request`].
pub enum Response {
    Selection(Selection),
    Document(Document),
}

impl Response {
    /// Returns Some(Selection) if this [`Response`] is a
    /// [`Response::Selection`], or None otherwise.
    pub fn get_selection(self) -> Option<Selection> {
        match self {
            Response::Selection(s) => Some(s),
            _ => None,
        }
    }

    /// Returns Some(Document) if this [`Response`] is a
    /// [`Response::Document`], or None otherwise.
    pub fn get_document(self) -> Option<Document> {
        match self {
            Response::Document(d) => Some(d),
            _ => None,
        }
    }
}
