use super::database::Database;
use super::operation_error::OperationError;
use super::query::Query;
use crate::archive::ArchiveParser;
use crate::schema::Document;

impl Database {
    pub fn create(&mut self, document: Document) -> Result<(), OperationError> {
        let bytes = document.serialize();
        self.io
            .write_block(bytes)
            .map_err(|e| OperationError::IOError(e))
    }

    pub fn find_one(&mut self, collection: u64, query: Query) -> Result<Document, OperationError> {
        let schema = self
            .collections
            .iter()
            .find(|s| s.id == collection)
            .ok_or(OperationError::UnknownSchemaIdentifier)?;
        self.io
            .reset_position()
            .map_err(|e| OperationError::IOError(e))?;
        loop {
            let block = self.io.next().map_err(|o| OperationError::IOError(o))?;
            let mut parser =
                ArchiveParser::new(schema.clone(), block, query.fields_of_interest.clone());
            return parser
                .read_document()
                .map_err(|e| OperationError::ParseError(e));
        }
    }

    pub fn find_many(
        &mut self,
        collection: u64,
        query: Query,
    ) -> Result<Vec<Document>, OperationError> {
        let schema = self
            .collections
            .iter()
            .find(|s| s.id == collection)
            .ok_or(OperationError::UnknownSchemaIdentifier)?;
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
            results.push(
                parser
                    .read_document()
                    .map_err(|e| OperationError::ParseError(e))?,
            );
        }
        Ok(results)
    }
}
