use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FrontendError {
    Redeclaration { identifier: String },
}

impl Display for FrontendError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FrontendError::Redeclaration { identifier } => {
                write!(formatter, "Redeclaration of identifier {}", identifier)
            }
        }
    }
}

impl Error for FrontendError {}
