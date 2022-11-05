use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    ReadError(std::io::Error),
    UnexpectedCharacter { position: usize, value: u8 },
    ArgumentCount,
    UnexpectedToken,
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
        }
    }
}

impl Error for ParseError {}
