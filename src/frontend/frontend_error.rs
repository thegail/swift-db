use crate::backend::OperationError;
use crate::language::ParseError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FrontendError {
    LanguageError(ParseError),
    OperationError(OperationError),
    SendError,
    RecieveError,
    TransactionRedeclaration(String),
    UnknownTransaction(String),
    SelectionRedeclaration(String),
    UnknownSelection(String),
}

impl Display for FrontendError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FrontendError::LanguageError(error) => {
                write!(formatter, "Language error: {}", error)
            }
            FrontendError::OperationError(error) => {
                write!(formatter, "Backend operation error: {}", error)
            }
            FrontendError::SendError => write!(formatter, "Error sending request to backend"),
            FrontendError::RecieveError => write!(formatter, "Error recieving from backend"),
            FrontendError::TransactionRedeclaration(identifier) => {
                write!(
                    formatter,
                    "Redeclaration of transaction identifier {}",
                    identifier
                )
            }
            FrontendError::UnknownTransaction(identifier) => {
                write!(formatter, "Unknown transaction identifier {}", identifier)
            }
            FrontendError::SelectionRedeclaration(identifier) => {
                write!(
                    formatter,
                    "Redeclaration of selection identifier {}",
                    identifier
                )
            }
            FrontendError::UnknownSelection(identifier) => {
                write!(formatter, "Unknown selection identifier {}", identifier)
            }
        }
    }
}

impl Error for FrontendError {}
