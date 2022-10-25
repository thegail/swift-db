use super::database::Database;
use super::operation_error::OperationError;
use super::query::Query;
use crate::archive::{ArchiveParser, ParseError};
use crate::schema::Document;

impl Database {
    pub fn create(&mut self, document: Document) -> Result<(), OperationError> {
        let bytes = document.serialize();
        self.io
            .write_block(bytes)
            .map_err(|e| OperationError::IOError(e))
    }

    pub fn find_one(&mut self, query: Query) -> Result<Document, OperationError> {
        let schema = self
            .collections
            .iter()
            .find(|s| s.id == query.collection)
            .ok_or(OperationError::UnknownSchemaIdentifier())?;
        self.io
            .reset_position()
            .map_err(|e| OperationError::IOError(e))?;
        loop {
            let block = self.io.next().map_err(|o| OperationError::IOError(o))?;
            let mut parser =
                ArchiveParser::new(schema.clone(), block, query.fields_of_interest.clone());
            let document_result = parser.read_document();
            match document_result {
                Err(ParseError::SchemaMismatch) => {}
                Err(error) => return Err(OperationError::ParseError(error)),
                Ok(document) => {
                    let matches = document.evaluate(&query.condition)?;
                    if matches {
                        return Ok(document);
                    }
                }
            }
        }
    }

    pub fn find_many(&mut self, query: Query) -> Result<Vec<Document>, OperationError> {
        let schema = self
            .collections
            .iter()
            .find(|s| s.id == query.collection)
            .ok_or(OperationError::UnknownSchemaIdentifier())?;
        self.io
            .reset_position()
            .map_err(|e| OperationError::IOError(e))?;
        let mut results = vec![];
        loop {
            let next = self.io.next();
            if let Err(error) = &next {
                if let std::io::ErrorKind::UnexpectedEof = error.kind() {
                    break;
                }
            }
            let block = next.unwrap();
            let mut parser =
                ArchiveParser::new(schema.clone(), block, query.fields_of_interest.clone());
            let document_result = parser.read_document();
            match document_result {
                Err(ParseError::SchemaMismatch) => {}
                Err(error) => return Err(OperationError::ParseError(error)),
                Ok(document) => {
                    let matches = document.evaluate(&query.condition)?;
                    if matches {
                        results.push(document)
                    }
                }
            }
        }
        Ok(results)
    }
}
