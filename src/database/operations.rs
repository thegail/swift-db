use super::database::Database;
use super::operation_error::OperationError;
use super::query::Query;
use crate::archive::ArchiveParser;
use crate::schema::Document;

impl Database {
    pub fn find(&mut self, collection: u64, query: Query) -> Result<Document, OperationError> {
        let schema = self
            .collections
            .iter()
            .find(|s| s.id == collection)
            .ok_or(OperationError::UnknownSchemaIdentifier)?;
        self.reader
            .reset_position()
            .map_err(|e| OperationError::ReadError(e))?;
        loop {
            let block = self
                .reader
                .next()
                .map_err(|o| OperationError::ReadError(o))?;
            let mut parser =
                ArchiveParser::new(schema.clone(), block, query.fields_of_interest.clone());
            return parser
                .read_document()
                .map_err(|e| OperationError::ParseError(e));
        }
    }
}
