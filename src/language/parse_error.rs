use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    ReadError(std::io::Error),
    UnexpectedCharacter(u8),
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self)
    }
}

impl Error for ParseError {}
