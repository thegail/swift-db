use super::frontend_error::FrontendError;
use super::transaction::Transaction;
use crate::{backend::Query, language::Statement};
use std::collections::HashMap;

pub struct Connection {
    pub transactions: HashMap<String, Transaction>,
}

impl Connection {
    pub fn execute_statement(&mut self, statement: Statement) {
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
            return Err(FrontendError::Redeclaration {
                identifier: transaction.clone(),
            });
        }
        self.transactions[transaction] = Transaction::new();
        Ok(())
    }

    fn acquire(&mut self, transaction: String) {
        todo!()
    }

    fn commit(&mut self, transaction: String) {
        todo!()
    }

    fn close(&mut self, transaction: String) {
        todo!()
    }

    fn select(&mut self, identifier: String, transaction: String, query: Query) {
        todo!()
    }

    fn read_all(&mut self, selection: String) {
        todo!()
    }
}
