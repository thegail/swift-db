use crate::archive::ParseError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum OperationError {
    ParseError(ParseError),
    IOError(std::io::Error),
    UnknownSchemaIdentifier,
}

impl Display for OperationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self)
    }
}
impl Error for OperationError {}
