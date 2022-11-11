use std::error::Error;
use std::fmt::{Display, Formatter};

/// An error during the startup of a [`Database`].
///
/// A lifecycle error cannot be recovered from and
/// should cause the process to exit.
///
/// [`Database`]: crate::database::Database
#[derive(Debug)]
pub enum LifecycleError {
    BackendError(std::io::Error),
    NetworkError(std::io::Error),
}

impl Display for LifecycleError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            LifecycleError::BackendError(e) => {
                write!(formatter, "Backend construction error: {}", e)
            }
            LifecycleError::NetworkError(e) => write!(formatter, "Network error: {}", e),
        }
    }
}

impl Error for LifecycleError {}
