use super::frontend_error::FrontendError;
use super::transaction::Transaction;
use crate::backend::{Operation, Query, Request};
use crate::language::{build_statement, parse, Response, Statement};
use crate::schema::Schema;
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};

/// A manager for a network connection with a client.
///
/// Instantiated by a [`Database`] instance's connection listener, a
/// `Connection` listens for data on its own thread, invokes the
/// [`language`] parser to build statements, then executes them. The
/// [`Transaction`] helper struct manages transaction state and helps in
/// the execution of statements. Requests are passed to the backend via
/// an MPSC channel, the response is serialized, and sent back over the
/// network stream.
///
/// [`language`]: crate::language
/// [`Database`]: crate::database::Database
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
    /// Creates a new [`Connection`] from a network stream.
    ///
    /// Takes a sender for sending requests to the backend and
    /// information about the Database's collections.
    pub fn new(stream: TcpStream, sender: Sender<Request>, collections: Vec<Schema>) -> Self {
        Self {
            stream,
            transactions: HashMap::new(),
            selection_map: HashMap::new(),
            sender,
            collections,
        }
    }

    /// Starts listening for data over the stream.
    ///
    /// Data is parsed via the [`language`] parser and expression
    /// builder, then executing it via [`execute_statement`]. The
    /// response is then recieved from the backend, transformed and
    /// serialized to be sent back to the client over the network
    /// stream.
    ///
    /// [`language`]: crate::language
    pub fn listen(&mut self) {
        loop {
            let response = parse(&mut BufReader::new(&mut self.stream))
                .map_err(FrontendError::LanguageError)
                .and_then(|tokens| {
                    // TODO buffer this read
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

/// Wrapper module for statement execution logic.
///
/// See [`execute_statement`].
///
/// [`execute_statement`]: crate::schema::Document#method.execute_statement
mod execute_statement {
    use super::*;
    use crate::schema::Document;

    impl Connection {
        /// Executes a language [`Statement`].
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
            let all_fields = selection.schema.fields.iter().map(|f| f.id).collect();
            self.sender
                .send(Request {
                    operation: Operation::Read {
                        selection: selection.clone(),
                        fields: all_fields,
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
