use crate::transfer::DeserializationError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    ReadError(std::io::Error),
    UnexpectedCharacter { position: usize, value: u8 },
    ArgumentCount,
    UnexpectedToken,
    UnknownIdentifier(String),
    TransferError(DeserializationError),
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
        }
    }
}

impl Error for ParseError {}
