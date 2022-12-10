use super::frontend_error::FrontendError;
use super::selection::Selection;
use super::transaction::Transaction;
use crate::backend::{Operation, Query, Request, Response as BackendResponse};
use crate::language::{build_statement, parse, Response, Statement};
use crate::schema::Schema;
use crate::util::LockType;
use std::collections::HashMap;
use std::io::{BufReader, BufWriter, Read, Write};
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
            let mut reader = BufReader::new(&self.stream);
            let response = parse(reader.by_ref())
                .map_err(FrontendError::LanguageError)
                .and_then(|tokens| {
                    build_statement(
                        &tokens,
                        &self.collections,
                        self.get_selection_map()?,
                        reader.by_ref(),
                    )
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
    use crate::{backend::Reference, schema::Document};

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
                    lock,
                    query,
                } => self.select(identifier, transaction, lock, query),
                Statement::Create {
                    identifier,
                    transaction,
                    document,
                } => self.create(identifier, transaction, document),
                Statement::ReadAll { selection } => self.read_all(selection),
                Statement::UpdateAll {
                    selection,
                    document,
                } => self.update_all(selection, document),
                Statement::Delete { selection } => self.delete(selection),
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

        fn acquire(&mut self, transaction_identifier: String) -> Result<Response, FrontendError> {
            let transaction_index = self.get_transaction_index(&transaction_identifier)?;
            let transaction = &mut self.transactions[transaction_index];
            let mut return_channels = Vec::with_capacity(transaction.selections.len());
            for selection in &transaction.selections {
                let (return_channel, return_reciever) = channel();
                self.sender
                    .send(Request {
                        operation: Operation::Acquire {
                            selection: selection.reference.clone(),
                            lock: selection.lock.clone(),
                        },
                        return_channel,
                    })
                    .or(Err(FrontendError::SendError))?;
                return_channels.push(return_reciever);
            }
            for reciever in return_channels {
                reciever
                    .recv()
                    .or(Err(FrontendError::RecieveError))?
                    .map_err(FrontendError::OperationError)?
                    .get_ok()
                    .ok_or(FrontendError::RecieveError)?;
            }
            transaction.acquire()?;
            Ok(Response::Acquired)
        }

        fn commit(&mut self, transaction: String) -> Result<Response, FrontendError> {
            self.close(transaction)
        }

        fn close(&mut self, transaction: String) -> Result<Response, FrontendError> {
            let index = self.get_transaction_index(&transaction)?;
            for selection in &self.transactions[index].selections {
                self.request(Operation::Release {
                    selection: selection.reference.clone(),
                })?;
            }
            self.transactions.remove(index);
            // TODO optimize
            let keys_to_remove: Vec<String> = self
                .selection_map
                .iter()
                .filter_map(|(k, (t, _))| (t == &transaction).then_some(k.clone()))
                .collect();
            for key in keys_to_remove {
                self.selection_map.remove(&key);
            }
            Ok(Response::Closed)
        }

        fn select(
            &mut self,
            identifier: String,
            transaction_identifier: String,
            lock: LockType,
            query: Query,
        ) -> Result<Response, FrontendError> {
            let transaction_index = self.get_transaction_index(&transaction_identifier)?;
            self.transactions[transaction_index].guard_selection()?;
            if self.selection_map.contains_key(&identifier) {
                return Err(FrontendError::SelectionRedeclaration(identifier));
            }
            let reference = self
                .request(Operation::FindOne { query })?
                .get_selection()
                .ok_or(FrontendError::RecieveError)?;
            let selection = Selection { reference, lock };
            self.create_selection(transaction_index, selection, identifier)?;
            Ok(Response::Selected)
        }

        fn create(
            &mut self,
            identifier: String,
            transaction_identifier: String,
            document: Document,
        ) -> Result<Response, FrontendError> {
            let transaction_index = self.get_transaction_index(&transaction_identifier)?;
            self.transactions[transaction_index].guard_action()?;
            if self.selection_map.contains_key(&identifier) {
                return Err(FrontendError::SelectionRedeclaration(identifier));
            }
            let reference = self
                .request(Operation::Create { document })?
                .get_selection()
                .ok_or(FrontendError::RecieveError)?;
            let selection = Selection {
                reference,
                lock: LockType::Write,
            };
            self.create_selection(transaction_index, selection, identifier)?;
            Ok(Response::Selected)
        }

        fn read_all(&mut self, selection: String) -> Result<Response, FrontendError> {
            let location = self
                .selection_map
                .get(&selection)
                .ok_or_else(|| FrontendError::UnknownSelection(selection.clone()))?;
            let transaction_index = self.get_transaction_index(&location.0)?;
            self.transactions[transaction_index].guard_action()?;
            let selection = &self.transactions[transaction_index].selections[location.1];
            let all_fields = selection
                .reference
                .schema
                .fields
                .iter()
                .map(|f| f.id)
                .collect();
            let document = self
                .request(Operation::Read {
                    selection: selection.reference.clone(),
                    fields: all_fields,
                })?
                .get_document()
                .ok_or(FrontendError::RecieveError)?;
            Ok(Response::Document(document))
        }

        fn update_all(
            &mut self,
            selection: String,
            document: Document,
        ) -> Result<Response, FrontendError> {
            let location = self
                .selection_map
                .get(&selection)
                .ok_or_else(|| FrontendError::UnknownSelection(selection.clone()))?;
            let transaction_index = self.get_transaction_index(&location.0)?;
            self.transactions[transaction_index].guard_action()?;
            let selection = &self.transactions[transaction_index].selections[location.1];
            self.request(Operation::Update {
                selection: selection.reference.clone(),
                fields: document.fields,
            })?
            .get_ok()
            .ok_or(FrontendError::RecieveError)?;
            Ok(Response::Updated)
        }

        fn delete(&mut self, selection: String) -> Result<Response, FrontendError> {
            let location = self
                .selection_map
                .get(&selection)
                .ok_or_else(|| FrontendError::UnknownSelection(selection.clone()))?;
            let transaction_index = self.get_transaction_index(&location.0)?;
            self.transactions[transaction_index].guard_action()?;
            let selection = &self.transactions[transaction_index].selections[location.1];
            self.request(Operation::Delete {
                selection: selection.reference.clone(),
            })?
            .get_ok()
            .ok_or(FrontendError::RecieveError)?;
            Ok(Response::Deleted)
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
            transaction_index: usize,
            selection: Selection,
            identifier: String,
        ) -> Result<(), FrontendError> {
            let transaction = &mut self.transactions[transaction_index];
            transaction.selections.push(selection);
            self.selection_map.insert(
                identifier,
                (
                    transaction.identifier.clone(),
                    transaction.selections.len() - 1,
                ),
            );
            Ok(())
        }

        fn get_transaction_index(
            &self,
            transaction_identifier: &String,
        ) -> Result<usize, FrontendError> {
            let index = self
                .transactions
                .iter()
                .position(|t| &t.identifier == transaction_identifier)
                .ok_or_else(|| FrontendError::UnknownTransaction(transaction_identifier.clone()))?;
            Ok(index)
        }

        pub fn get_selection_map(&self) -> Result<HashMap<String, &Reference>, FrontendError> {
            let entries: Result<HashMap<String, &Reference>, FrontendError> = self
                .selection_map
                .iter()
                .map(|(key, (transaction_id, index))| {
                    Ok((
                        key.clone(),
                        &self.transactions[self.get_transaction_index(transaction_id)?].selections
                            [*index]
                            .reference,
                    ))
                })
                .collect();
            entries
        }
    }
}
