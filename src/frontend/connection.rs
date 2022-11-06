use super::transaction::Transaction;
use crate::language::Statement;
use std::collections::HashMap;

pub struct Connection {
    pub transactions: HashMap<String, Transaction>,
}

impl Connection {
    fn execute_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Open { transaction } => {
                todo!()
            }
            Statement::Acquire { transaction } => {
                todo!()
            }
            Statement::Commit { transaction } => {
                todo!()
            }
            Statement::Close { transaction } => {
                todo!()
            }
            Statement::Select {
                identifier,
                transaction,
                query,
            } => {
                todo!()
            }
            Statement::ReadAll { selection } => {
                todo!()
            }
            _ => todo!(),
        }
    }
}
