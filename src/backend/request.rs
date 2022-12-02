use crate::backend::{OperationError, Query, Selection};
use crate::schema::{Document, FieldInstance};
use std::sync::mpsc::Sender;

/// A request for the [`Backend`] to execute some [`Operation`].
///
/// [`Backend`]: crate::backend::Backend
pub struct Request {
    /// The [`Operation`] to execute.
    pub operation: Operation,
    /// A sender to send the [`Response`] back to the frontend.
    pub return_channel: Sender<Result<Response, OperationError>>,
}

/// An operation for the [`Backend`] to execute.
///
/// [`Backend`]: crate::backend::Backend
pub enum Operation {
    /// Find one [`Document`] in a collection.
    ///
    /// See [`Query`]. Returns a [`Response::Selection`].
    FindOne { query: Query },
    /// Wait to acquire a lock on a [`Selection`]. Takes the
    /// selection to wait for the lock on. Returns a
    /// [`Response::Ok`].
    Acquire { selection: Selection },
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
    /// Update the [`Document`] referred to by `selection`,
    /// replacing its existin fields with new [`FieldInstance`]s.
    ///
    /// Returns a [`Response::Ok`].
    Update {
        selection: Selection,
        fields: Vec<FieldInstance>,
    },
    /// Delete the [`Document`] referred to by `selection`.
    ///
    /// Returns a [`Response::Ok`].
    Delete { selection: Selection },
    /// Release the lock on [`Selection`].
    Release { selection: Selection },
}

/// A response to a [`Request`].
pub enum Response {
    Selection(Selection),
    Document(Document),
    Ok,
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

    pub fn get_ok(self) -> Option<()> {
        match self {
            Response::Ok => Some(()),
            _ => None,
        }
    }
}
