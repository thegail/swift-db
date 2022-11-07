use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum LifecycleError {
    BackendError(std::io::Error),
}

impl Display for LifecycleError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            LifecycleError::BackendError(e) => {
                write!(formatter, "Backend construction error: {}", e)
            }
        }
    }
}

impl Error for LifecycleError {}