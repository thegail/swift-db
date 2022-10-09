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

    pub fn find(&mut self, collection: u64, query: Query) -> Result<Document, OperationError> {
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
}
