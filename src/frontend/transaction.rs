use crate::backend::Selection;
use std::collections::HashMap;

/// A helper struct for managing transaction state.
///
/// Instantiated by a [`Connection`][crate::frontend::Connection]
/// whenever a new transaction is opened. Helper methods are
/// used when running queries on this transaction.
pub struct Transaction {
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
