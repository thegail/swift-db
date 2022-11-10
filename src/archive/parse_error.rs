use std::error::Error;
use std::fmt::{Display, Formatter};

/// An error raised during parsing with an
/// [`ArchiveParser`][crate::archive::ArchiveParser].
///
/// This error gets wrapped in an
/// [`OperationError`][crate::backend::OperationError].
#[derive(Debug)]
pub enum ParseError {
    SchemaMismatch,
    UnknownFieldIdentifier,
    UnknownCaseIdentifier,
    InvalidString,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        let string = match self {
            ParseError::SchemaMismatch => "Mismatched schema in archive parser",
            ParseError::UnknownFieldIdentifier => "Unknown field in archive",
            ParseError::UnknownCaseIdentifier => "Unkown enum case in archive",
            ParseError::InvalidString => "Invalid UTF-8 string in archive",
        };
        write!(formatter, "{}", string)
    }
}
impl Error for ParseError {}
