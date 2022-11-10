use super::frontend_error::FrontendError;
use super::transaction::Transaction;
use crate::backend::{Operation, Query, Request};
use crate::language::{build_statement, parse, Response, Statement};
use crate::schema::Schema;
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};

pub struct Connection {
    stream: TcpStream,
    transactions: HashMap<String, Transaction>,
    // TODO this is incredibly stupid but it works...im tired
    selection_map: HashMap<String, String>,
    sender: Sender<Request>,
    // TODO something better here
    collections: Vec<Schema>,
}

impl Connection {
    pub fn new(stream: TcpStream, sender: Sender<Request>, collections: Vec<Schema>) -> Self {
        Self {
            stream,
            transactions: HashMap::new(),
            selection_map: HashMap::new(),
            sender,
            collections,
        }
    }

    pub fn listen(&mut self) {
        loop {
            let response = parse(&mut BufReader::new(&mut self.stream))
                .map_err(FrontendError::LanguageError)
                .and_then(|tokens| {
                    build_statement(&tokens, &self.collections, self.stream.by_ref())
                        .map_err(FrontendError::LanguageError)
                })
                .and_then(|statement| self.execute_statement(statement));
            let mut writer = BufWriter::new(&mut self.stream);
            let write_result = match response {
                Ok(response) => response.serialize(&mut writer),
                // TODO escape this somehow
                Err(error) => writeln!(writer, "(error \"{}\")", error),
            };
            if write_result.and_then(|_| writer.flush()).is_err() {
                break;
            }
        }
    }
}

mod execute_statement {
    use crate::schema::Document;

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
                Statement::Create {
                    identifier,
                    transaction,
                    document,
                } => self.create(identifier, transaction, document),
                Statement::ReadAll { selection } => self.read_all(selection),
                // _ => todo!(),
            }
        }

        fn open(&mut self, transaction: String) -> Result<Response, FrontendError> {
            if self.transactions.contains_key(&transaction) {
                return Err(FrontendError::TransactionRedeclaration(transaction));
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
            transaction_identifier: String,
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
                .get_mut(&transaction_identifier)
                .ok_or_else(|| FrontendError::UnknownTransaction(transaction_identifier.clone()))?;
            transaction.selections.insert(
                identifier.clone(),
                result.get_selection().ok_or(FrontendError::RecieveError)?,
            );
            self.selection_map
                .insert(identifier, transaction_identifier);
            Ok(Response::Selected)
        }

        fn create(
            &mut self,
            identifier: String,
            transaction_identifier: String,
            document: Document,
        ) -> Result<Response, FrontendError> {
            if self.selection_map.contains_key(&identifier) {
                return Err(FrontendError::SelectionRedeclaration(identifier));
            }
            let (returner, return_reciever) = channel();
            self.sender
                .send(Request {
                    operation: Operation::Create { document },
                    return_channel: returner,
                })
                .or(Err(FrontendError::SendError))?;
            let result = return_reciever
                .recv()
                .or(Err(FrontendError::RecieveError))?
                .map_err(FrontendError::OperationError)?;
            let transaction = self
                .transactions
                .get_mut(&transaction_identifier)
                .ok_or_else(|| FrontendError::UnknownTransaction(transaction_identifier.clone()))?;
            transaction.selections.insert(
                identifier.clone(),
                result.get_selection().ok_or(FrontendError::RecieveError)?,
            );
            self.selection_map
                .insert(identifier, transaction_identifier);
            Ok(Response::Selected)
        }

        fn read_all(&mut self, selection: String) -> Result<Response, FrontendError> {
            let selection = &self.transactions[self
                .selection_map
                .get(&selection)
                .ok_or_else(|| FrontendError::UnknownSelection(selection.clone()))?]
            .selections[&selection];
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
