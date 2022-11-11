use crate::backend::Selection;
use std::collections::HashMap;

/// A helper struct for managing transaction state.
///
/// Instantiated by a [`Connection`] whenever a new transaction
/// is opened. Helper methods are used when running queries on
/// this transaction.
///
/// [`Connection`]: crate::frontend::Connection
pub struct Transaction {
    /// A map between selection identifiers and their
    /// corresponding [`Selection`]s.
    pub selections: HashMap<String, Selection>,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new() -> Self {
        Transaction {
            selections: HashMap::new(),
        }
    }
}
