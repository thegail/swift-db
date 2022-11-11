use super::frontend_error::FrontendError;
use super::transaction::Transaction;
use crate::backend::{Operation, Query, Request, Response as BackendResponse};
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
    transactions: Vec<Transaction>,
    selection_map: HashMap<String, (String, usize)>,
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
            transactions: Vec::new(),
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
    use crate::{backend::Selection, schema::Document};

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
            if self
                .transactions
                .iter()
                .any(|t| t.identifier == transaction)
            {
                return Err(FrontendError::TransactionRedeclaration(transaction));
            }
            self.transactions.push(Transaction::new(transaction));
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
            let selection = self
                .request(Operation::FindOne { query })?
                .get_selection()
                .ok_or(FrontendError::RecieveError)?;
            self.create_selection(transaction_identifier, selection, identifier)?;
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
            let selection = self
                .request(Operation::Create { document })?
                .get_selection()
                .ok_or(FrontendError::RecieveError)?;
            self.create_selection(transaction_identifier, selection, identifier)?;
            Ok(Response::Selected)
        }

        fn read_all(&mut self, selection: String) -> Result<Response, FrontendError> {
            let location = self
                .selection_map
                .get(&selection)
                .ok_or_else(|| FrontendError::UnknownSelection(selection.clone()))?;
            let selection = &self
                .transactions
                .iter()
                .find(|f| f.identifier == location.0)
                .unwrap()
                .selections[location.1];
            let all_fields = selection.schema.fields.iter().map(|f| f.id).collect();
            let document = self
                .request(Operation::Read {
                    selection: selection.clone(),
                    fields: all_fields,
                })?
                .get_document()
                .ok_or(FrontendError::RecieveError)?;
            Ok(Response::Document(document))
        }

        fn request(&self, operation: Operation) -> Result<BackendResponse, FrontendError> {
            let (returner, return_reciever) = channel();
            self.sender
                .send(Request {
                    operation,
                    return_channel: returner,
                })
                .or(Err(FrontendError::SendError))?;
            let result = return_reciever
                .recv()
                .or(Err(FrontendError::RecieveError))?
                .map_err(FrontendError::OperationError)?;
            Ok(result)
        }

        fn create_selection(
            &mut self,
            transaction_identifier: String,
            selection: Selection,
            identifier: String,
        ) -> Result<(), FrontendError> {
            let transaction = self
                .transactions
                .iter_mut()
                .find(|t| t.identifier == transaction_identifier)
                .ok_or_else(|| FrontendError::UnknownTransaction(transaction_identifier.clone()))?;
            transaction.selections.push(selection);
            self.selection_map.insert(
                identifier,
                (transaction_identifier, transaction.selections.len() - 1),
            );
            Ok(())
        }
    }
}
