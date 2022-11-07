use crate::backend::OperationError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FrontendError {
    Redeclaration { identifier: String },
    OperationError(OperationError),
}

impl Display for FrontendError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FrontendError::Redeclaration { identifier } => {
                write!(formatter, "Redeclaration of identifier {}", identifier)
            }
            FrontendError::OperationError(error) => {
                write!(formatter, "Backend operation error: {}", error)
            }
        }
    }
}

impl Error for FrontendError {}
