use crate::transfer::DeserializationError;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// An error raised when transforming a byte stream into a [`Statement`].
///
/// This gets wrapped in a `FrontendError` before being passed back to
/// the client.
///
/// [`Statement`]: crate::language::Statement
#[derive(Debug)]
pub enum ParseError {
    ReadError(std::io::Error),
    UnexpectedCharacter { position: usize, value: u8 },
    ArgumentCount,
    UnexpectedToken,
    UnknownIdentifier(String),
    TransferError(DeserializationError),
    NumericError,
    UnexpectedEndOfInput,
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::ReadError(err) => write!(formatter, "Read error: {}", err),
            ParseError::UnexpectedCharacter {
                position,
                value: character,
            } => {
                write!(
                    formatter,
                    "Unexpected character {} at position {}",
                    *character as char, position,
                )
            }
            ParseError::ArgumentCount => write!(formatter, "Incorrect number of arguments"),
            ParseError::UnexpectedToken => write!(formatter, "Unexpected token"),
            ParseError::UnknownIdentifier(identifier) => {
                write!(formatter, "Unknown identifier {}", identifier)
            }
            ParseError::TransferError(e) => write!(formatter, "Transfer parse error: {}", e),
            ParseError::NumericError => write!(formatter, "Error parsing numeric"),
            ParseError::UnexpectedEndOfInput => write!(formatter, "Unexpected end of input"),
        }
    }
}

impl Error for ParseError {}
