use crate::backend::OperationError;
use crate::language::ParseError;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// An error executing a statement.
///
/// `FrontendError`s are serialized and sent to the client.
#[derive(Debug)]
pub enum FrontendError {
    LanguageError(ParseError),
    OperationError(OperationError),
    SendError,
    RecieveError,
    TransactionState,
    TransactionRedeclaration(String),
    UnknownTransaction(String),
    SelectionRedeclaration(String),
    UnknownSelection(String),
    MissingCache,
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
            FrontendError::TransactionState => write!(
                formatter,
                "Action not permitted in current transaction state"
            ),
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
            FrontendError::MissingCache => {
                write!(formatter, "Missing cached document")
            }
        }
    }
}

impl Error for FrontendError {}
