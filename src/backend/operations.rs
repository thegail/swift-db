use super::backend::Backend;
use super::operation_error::OperationError;
use super::query::Query;
use super::selection::Selection;
use crate::archive::{ArchiveParser, ParseError};
use crate::schema::{Document, Schema};

impl Backend {
    pub fn create(&mut self, document: Document) -> Result<(), OperationError> {
        let bytes = document.serialize();
        self.io.write_block(bytes).map_err(OperationError::IOError)
    }

    pub fn find_one(&mut self, query: Query) -> Result<Selection, OperationError> {
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

    pub fn find_many(&mut self, query: Query) -> Result<Vec<usize>, OperationError> {
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
        Ok(results)
    }

    pub fn read(
        &mut self,
        pos: usize,
        schema: &Schema,
        fields: Vec<u16>,
    ) -> Result<Document, OperationError> {
        let block = self
            .io
            .read_at_position(pos as u64)
            .map_err(OperationError::IOError)?;
        let document = ArchiveParser::new(schema.clone(), block, fields)
            .read_document()
            .map_err(OperationError::ParseError)?;
        Ok(document)
    }

    pub fn read_many(
        &mut self,
        pos: Vec<usize>,
        schema: &Schema,
        fields: Vec<u16>,
    ) -> Result<Vec<Document>, OperationError> {
        pos.iter()
            .map(|p| {
                let block = self
                    .io
                    .read_at_position(*p as u64)
                    .map_err(OperationError::IOError)?;
                let document = ArchiveParser::new(schema.clone(), block, fields.clone())
                    .read_document()
                    .map_err(OperationError::ParseError)?;
                Ok(document)
            })
            .collect()
    }
}
