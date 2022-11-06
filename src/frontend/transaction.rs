use crate::backend::Selection;
use crate::language::Statement;
use std::collections::HashMap;

pub struct Transaction {
    pub selections: HashMap<String, Selection>,
}

impl Transaction {
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
