use super::frontend_error::FrontendError;
use super::transaction::Transaction;
use crate::backend::{Operation, Query, Request};
use crate::language::Statement;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};

pub struct Connection {
    transactions: HashMap<String, Transaction>,
    // TODO this is incredibly stupid but it works...im tired
    selection_map: HashMap<String, String>,
    sender: Sender<Request>,
}

impl Connection {
    pub fn new(sender: Sender<Request>) -> Self {
        Self {
            transactions: HashMap::new(),
            sender,
        }
    }

    pub fn execute_statement(&mut self, statement: Statement) -> Result<(), FrontendError> {
        match statement {
            Statement::Open { transaction } => self.open(transaction),
            Statement::Acquire { transaction } => self.acquire(transaction),
            Statement::Commit { transaction } => self.commit(transaction),
            Statement::Close { transaction } => self.close(transaction),
            Statement::Select {
                identifier,
                transaction,
                query,
            } => self.select(identifier, transaction, query),
            Statement::ReadAll { selection } => self.read_all(selection),
            _ => todo!(),
        }
    }

    fn open(&mut self, transaction: String) -> Result<(), FrontendError> {
        if self.transactions.contains_key(&transaction) {
            return Err(FrontendError::TransactionRedeclaration(transaction.clone()));
        }
        self.transactions.insert(transaction, Transaction::new());
        Ok(())
    }

    fn acquire(&mut self, _transaction: String) -> Result<(), FrontendError> {
        todo!()
    }

    fn commit(&mut self, _transaction: String) -> Result<(), FrontendError> {
        todo!()
    }

    fn close(&mut self, _transaction: String) -> Result<(), FrontendError> {
        todo!()
    }

    fn select(
        &mut self,
        identifier: String,
        transaction: String,
        query: Query,
    ) -> Result<(), FrontendError> {
        if self.selection_map.contains_key(&identifier) {
            return Err(FrontendError::SelectionRedeclaration(identifier));
        }
        let (returner, return_reciever) = channel();
        self.sender
            .send(Request {
                operation: Operation::FindOne { query },
                return_channel: returner,
            })
            .or(Err(FrontendError::SendError))?;
        let result = return_reciever
            .recv()
            .or(Err(FrontendError::RecieveError))?
            .map_err(FrontendError::OperationError)?;
        let transaction = self
            .transactions
            .get_mut(&transaction)
            .ok_or(FrontendError::UnknownTransaction(transaction))?;
        transaction.selections.insert(
            identifier,
            result.get_selection().ok_or(FrontendError::RecieveError)?,
        );
        Ok(())
    }

    fn read_all(&mut self, selection: String) -> Result<(), FrontendError> {
        let 
    }
}
