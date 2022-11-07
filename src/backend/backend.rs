use super::operation_error::OperationError;
use super::query::Query;
use super::selection::{ManySelection, Selection};
use super::{Operation, Request, Response};
use crate::archive::BlockFileIO;
use crate::archive::{ArchiveParser, ParseError};
use crate::schema::{Document, Schema};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::sync::mpsc::Receiver;

pub struct Backend {
    io: BlockFileIO,
    collections: Vec<Schema>,
    document_cache: HashMap<usize, Document>,
    reciever: Receiver<Request>,
}

impl Backend {
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
            reciever,
        })
    }

    pub fn listen(&mut self) {
        while let Ok(request) = self.reciever.recv() {
            let result = self.execute_operation(request.operation);
        }
    }
}

mod operations {
    use super::*;
    impl Backend {
        pub fn execute_operation(
            &mut self,
            operation: Operation,
        ) -> Result<Response, OperationError> {
            match operation {
                Operation::FindOne { query } => Ok(Response::Selection(self.find_one(query)?)),
                Operation::Read { selection, fields } => {
                    Ok(Response::Document(self.read(selection, fields)?))
                }
            }
        }

        fn create(&mut self, document: Document) -> Result<(), OperationError> {
            let bytes = document.serialize();
            self.io.write_block(bytes).map_err(OperationError::IOError)
        }

        fn find_one(&mut self, query: Query) -> Result<Selection, OperationError> {
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
                            return Ok(Selection {
                                position,
                                schema: schema.clone(),
                            });
                        }
                    }
                }
            }
        }

        fn find_many(&mut self, query: Query) -> Result<ManySelection, OperationError> {
            let schema = self
                .collections
                .iter()
                .find(|s| s.id == query.collection)
                .ok_or(OperationError::UnknownSchemaIdentifier)?;
            self.io.reset_position().map_err(OperationError::IOError)?;
            let mut results = vec![];
            loop {
                let next = self.io.next();
                if let Err(error) = &next {
                    if let std::io::ErrorKind::UnexpectedEof = error.kind() {
                        break;
                    }
                }
                let (position, block) = next.unwrap();
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
                        self.document_cache.insert(position, document);
                        if matches {
                            results.push(position)
                        }
                    }
                }
            }
            Ok(ManySelection {
                schema: schema.clone(),
                positions: results,
            })
        }

        fn read(
            &mut self,
            selection: Selection,
            fields: Vec<u16>,
        ) -> Result<Document, OperationError> {
            let block = self
                .io
                .read_at_position(selection.position as u64)
                .map_err(OperationError::IOError)?;
            let document = ArchiveParser::new(selection.schema, block, fields)
                .read_document()
                .map_err(OperationError::ParseError)?;
            Ok(document)
        }

        fn read_many(
            &mut self,
            selection: ManySelection,
            fields: Vec<u16>,
        ) -> Result<Vec<Document>, OperationError> {
            selection
                .positions
                .iter()
                .map(|p| {
                    let block = self
                        .io
                        .read_at_position(*p as u64)
                        .map_err(OperationError::IOError)?;
                    let document =
                        ArchiveParser::new(selection.schema.clone(), block, fields.clone())
                            .read_document()
                            .map_err(OperationError::ParseError)?;
                    Ok(document)
                })
                .collect()
        }
    }
}
