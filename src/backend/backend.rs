use super::lock::Lock;
use crate::archive::{ArchiveParser, BlockFileIO, ParseError};
use crate::backend::{Operation, OperationError, Query, Reference, Request, Response};
use crate::schema::{Document, FieldInstance, Schema};
use crate::util::{BlockPosition, FieldID, LockType};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::sync::mpsc::{Receiver, Sender};

/// The core of the databse's read/write logic.
///
/// Each [`Database`] instance creates and owns one `Backend`, which
/// responds to [`Request`]s from various [`frontend`]s via an MPSC
/// channel. A `Backend`'s interface with the disk is through a
/// [`BlockFileIO`] manager, which reads and writes blocks (documents)
/// from the data file. The `Backend` uses [`ArchiveParser`]s to parse
/// documents in the [`archive`] binary serialization format.
///
/// [`Database`]: crate::database::Database
/// [`frontend`]: crate::frontend
/// [`archive`]: crate::archive
pub struct Backend {
    io: BlockFileIO,
    collections: Vec<Schema>,
    document_cache: HashMap<usize, Document>,
    locks: HashMap<usize, Lock>,
    reciever: Receiver<Request>,
}

impl Backend {
    /// Creates a new [`Backend`] instance.
    ///
    /// Accepts a list of [`Schema`] definitions, a path at which
    /// the data file is stored, and the recieving end of the
    /// channel for recieving [`Request`]s.
    pub fn new(
        path: String,
        collections: Vec<Schema>,
        reciever: Receiver<Request>,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            io: BlockFileIO::new(
                File::open(&path)?,
                OpenOptions::new().read(true).write(true).open(&path)?,
            ),
            collections,
            document_cache: HashMap::new(),
            locks: HashMap::new(),
            reciever,
        })
    }

    /// Begins the [`Backend`]'s request execution cycle.
    ///
    /// When a [`Database`] calls this function, the [`Backend`] instance
    /// will continuously recieve [`Request`]s from its channel reciever,
    /// execute them (see [`Backend::execute_operation`]), and return a
    /// [`Response`] back to the [`frontend`] through the return channel.
    ///
    /// [`Database`]: crate::database::Database
    /// [`frontend`]: crate::frontend
    pub fn listen(&mut self) {
        while let Ok(request) = self.reciever.recv() {
            if let Operation::Acquire { selection, lock } = request.operation {
                self.acquire(&selection, lock, request.return_channel);
            } else {
                let result = self.execute_operation(request.operation);
                let send_result = request.return_channel.send(result);
                if let Err(error) = send_result {
                    println!("Send error: {}", error);
                }
            }
        }
    }
}

/// Execute [`Operation`]s from [`Request`]s.
///
/// See [`Backend::execute_operation`].
mod operations {
    use super::*;

    impl Backend {
        /// Execute an [`Operation`] from a [`Request`].
        ///
        /// Returns a [`Response`] or an [`OperationError`].
        pub fn execute_operation(
            &mut self,
            operation: Operation,
        ) -> Result<Response, OperationError> {
            match operation {
                Operation::FindOne { query } => Ok(Response::Selection(self.find_one(query)?)),
                Operation::Acquire {
                    selection: _,
                    lock: _,
                } => unreachable!(),
                Operation::Create { document } => {
                    let selection = self.create(document)?;
                    Ok(Response::Selection(selection))
                }
                Operation::Read { selection, fields } => {
                    Ok(Response::Document(self.read(selection, fields)?))
                }
                Operation::Update { selection, fields } => {
                    self.update(selection, fields)?;
                    Ok(Response::Ok)
                }
                Operation::Delete { selection } => {
                    self.delete(selection)?;
                    Ok(Response::Ok)
                }
                Operation::Release { selection, lock } => {
                    self.release(selection, lock);
                    Ok(Response::Ok)
                }
            }
        }

        /// Executes an acquisition operation.
        ///
        /// Returns a [`Response::Ok`] after acquisition.
        pub fn acquire(
            &mut self,
            selection: &Reference,
            lock: LockType,
            return_sender: Sender<Result<Response, OperationError>>,
        ) {
            // TODO optimize order of acquisition
            // TODO optimize queueing system (linked list?)
            let current = self.locks.get_mut(&selection.position);
            if let Some(current) = current {
                current.queue(return_sender, lock);
            } else {
                self.locks.insert(selection.position, Lock::new(lock));
                return_sender.send(Ok(Response::Ok)).unwrap_or(());
            }
        }

        fn release(&mut self, selection: Reference, lock: LockType) {
            let entry = self.locks.get_mut(&selection.position).unwrap();
            entry.release(&lock);
        }

        fn create(&mut self, document: Document) -> Result<Reference, OperationError> {
            let bytes = document.serialize();
            let position = self
                .io
                .write_block(bytes)
                .map_err(OperationError::IOError)?;
            let selection = Reference {
                position,
                schema: document.schema,
            };
            Ok(selection)
        }

        fn find_one(&mut self, query: Query) -> Result<Reference, OperationError> {
            let schema = self
                .collections
                .iter()
                .find(|s| s.id == query.collection)
                .ok_or(OperationError::UnknownSchemaIdentifier)?;
            self.io.reset_position().map_err(OperationError::IOError)?;
            loop {
                let (position, block) = self.io.next().map_err(OperationError::IOError)?;
                let mut parser = ArchiveParser::new(
                    schema.clone(),
                    block,
                    // TODO optimize
                    schema.fields.iter().map(|f| f.id).collect(),
                );
                let document_result = parser.read_document();
                match document_result {
                    Err(ParseError::SchemaMismatch) => {}
                    Err(error) => return Err(OperationError::ParseError(error)),
                    Ok(document) => {
                        let matches = document.evaluate(&query.condition)?;
                        if matches {
                            self.document_cache.insert(position, document);
                            return Ok(Reference {
                                position,
                                schema: schema.clone(),
                            });
                        }
                    }
                }
            }
        }

        // fn find_many(&mut self, query: Query) -> Result<ManySelection, OperationError> {
        //     let schema = self
        //         .collections
        //         .iter()
        //         .find(|s| s.id == query.collection)
        //         .ok_or(OperationError::UnknownSchemaIdentifier)?;
        //     self.io.reset_position().map_err(OperationError::IOError)?;
        //     let mut results = vec![];
        //     loop {
        //         let next = self.io.next();
        //         if let Err(error) = &next {
        //             if let std::io::ErrorKind::UnexpectedEof = error.kind() {
        //                 break;
        //             }
        //         }
        //         let (position, block) = next.unwrap();
        //         let mut parser = ArchiveParser::new(
        //             schema.clone(),
        //             block,
        //             // TODO optimize
        //             schema.fields.iter().map(|f| f.id).collect(),
        //         );
        //         let document_result = parser.read_document();
        //         match document_result {
        //             Err(ParseError::SchemaMismatch) => {}
        //             Err(error) => return Err(OperationError::ParseError(error)),
        //             Ok(document) => {
        //                 let matches = document.evaluate(&query.condition)?;
        //                 self.document_cache.insert(position, document);
        //                 if matches {
        //                     results.push(position)
        //                 }
        //             }
        //         }
        //     }
        //     Ok(ManySelection {
        //         schema: schema.clone(),
        //         positions: results,
        //     })
        // }

        fn read(
            &mut self,
            selection: Reference,
            fields: Vec<FieldID>,
        ) -> Result<Document, OperationError> {
            let block = self
                .io
                .read_at_position(selection.position as BlockPosition)
                .map_err(OperationError::IOError)?;
            let document = ArchiveParser::new(selection.schema, block, fields)
                .read_document()
                .map_err(OperationError::ParseError)?;
            Ok(document)
        }

        // fn read_many(
        //     &mut self,
        //     selection: ManySelection,
        //     fields: Vec<u16>,
        // ) -> Result<Vec<Document>, OperationError> {
        //     selection
        //         .positions
        //         .iter()
        //         .map(|p| {
        //             let block = self
        //                 .io
        //                 .read_at_position(*p as u64)
        //                 .map_err(OperationError::IOError)?;
        //             let document =
        //                 ArchiveParser::new(selection.schema.clone(), block, fields.clone())
        //                     .read_document()
        //                     .map_err(OperationError::ParseError)?;
        //             Ok(document)
        //         })
        //         .collect()
        // }

        fn update(
            &mut self,
            selection: Reference,
            fields: Vec<FieldInstance>,
        ) -> Result<(), OperationError> {
            // TODO optimize
            self.delete(selection.clone())?;
            self.create(Document {
                schema: selection.schema,
                fields,
            })?;
            Ok(())
        }

        fn delete(&mut self, selection: Reference) -> Result<(), OperationError> {
            self.io
                .remove_block(selection.position)
                .map_err(OperationError::IOError)?;
            Ok(())
        }
    }
}
