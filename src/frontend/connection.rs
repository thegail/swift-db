use super::frontend_error::FrontendError;
use super::transaction::Transaction;
use crate::backend::{Operation, Query, Request};
use crate::language::{build_statement, parse, Response, Statement};
use std::collections::HashMap;
use std::io::{BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};

pub struct Connection {
    stream: TcpStream,
    transactions: HashMap<String, Transaction>,
    // TODO this is incredibly stupid but it works...im tired
    selection_map: HashMap<String, String>,
    sender: Sender<Request>,
}

impl Connection {
    pub fn new(stream: TcpStream, sender: Sender<Request>) -> Self {
        Self {
            stream,
            transactions: HashMap::new(),
            selection_map: HashMap::new(),
            sender,
        }
    }

    pub fn listen(&mut self) {
        loop {
            let response = parse(&mut BufReader::new(&mut self.stream))
                .map_err(FrontendError::LanguageError)
                .and_then(|tokens| build_statement(&tokens).map_err(FrontendError::LanguageError))
                .and_then(|statement| self.execute_statement(statement));
            let write_result = match response {
                Ok(response) => writeln!(self.stream, "{}", response.serialize()),
                // TODO escape this somehow
                Err(error) => writeln!(self.stream, "(error \"{}\")", error),
            };
            if let Err(_) = write_result {
                break;
            }
        }
    }
}

mod execute_statement {
    use super::*;

    impl Connection {
        pub fn execute_statement(
            &mut self,
            statement: Statement,
        ) -> Result<Response, FrontendError> {
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

        fn open(&mut self, transaction: String) -> Result<Response, FrontendError> {
            if self.transactions.contains_key(&transaction) {
                return Err(FrontendError::TransactionRedeclaration(transaction.clone()));
            }
            self.transactions.insert(transaction, Transaction::new());
            Ok(Response::Opened)
        }

        fn acquire(&mut self, _transaction: String) -> Result<Response, FrontendError> {
            todo!()
        }

        fn commit(&mut self, _transaction: String) -> Result<Response, FrontendError> {
            todo!()
        }

        fn close(&mut self, _transaction: String) -> Result<Response, FrontendError> {
            todo!()
        }

        fn select(
            &mut self,
            identifier: String,
            transaction: String,
            query: Query,
        ) -> Result<Response, FrontendError> {
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
            Ok(Response::Selected)
        }

        fn read_all(&mut self, selection: String) -> Result<Response, FrontendError> {
            let selection =
                &self.transactions[&self.selection_map[&selection]].selections[&selection];
            let (returner, return_reciever) = channel();
            self.sender
                .send(Request {
                    operation: Operation::Read {
                        selection: selection.clone(),
                        fields: vec![],
                    },
                    return_channel: returner,
                })
                .or(Err(FrontendError::SendError))?;
            let result = return_reciever
                .recv()
                .or(Err(FrontendError::RecieveError))?
                .map_err(FrontendError::OperationError)?;
            Ok(Response::Document(
                result.get_document().ok_or(FrontendError::RecieveError)?,
            ))
        }
    }
}
