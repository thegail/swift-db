use super::frontend_error::FrontendError;
use crate::backend::Selection;

/// A helper struct for managing transaction state.
///
/// Instantiated by a [`Connection`] whenever a new transaction
/// is opened. Helper methods are used when running queries on
/// this transaction.
///
/// [`Connection`]: crate::frontend::Connection
pub struct Transaction {
    /// The language identifier referring to this transaction.
    pub identifier: String,
    pub selections: Vec<Selection>,
    state: State,
}

impl Transaction {
    /// Creates a new transaction.
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            selections: Vec::new(),
            state: State::Selection,
        }
    }

    pub fn guard_selection(&self) -> Result<(), FrontendError> {
        match self.state {
            State::Selection => Ok(()),
            _ => Err(FrontendError::TransactionState),
        }
    }

    pub fn acquire(&mut self) -> Result<(), FrontendError> {
        self.guard_selection()?;
        self.state = State::Action;
        Ok(())
    }

    pub fn guard_action(&self) -> Result<(), FrontendError> {
        match self.state {
            State::Action => Ok(()),
            _ => Err(FrontendError::TransactionState),
        }
    }
}

enum State {
    Selection,
    Action,
}
