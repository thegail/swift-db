use crate::archive::ParseError;
use crate::schema::FieldType;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// An error raised while executing a backend [`Request`].
///
/// This gets wrapped in a [`FrontendError`] and sent to the client.
///
/// [`Request`]: crate::backend::Request
/// [`FrontendError`]: crate::frontend::frontend_error::FrontendError
#[derive(Debug)]
pub enum OperationError {
    ParseError(ParseError),
    IOError(std::io::Error),
    UnknownSchemaIdentifier,
    UnknownFieldIdentifier,
    ExpressionTypeMismatch { left: FieldType, right: FieldType },
    InvalidExpressionType,
}

impl Display for OperationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            OperationError::ParseError(err) => write!(formatter, "Parse error: {}", err),
            OperationError::IOError(err) => write!(formatter, "Archive read error: {}", err),
            OperationError::UnknownSchemaIdentifier => {
                write!(formatter, "Unknown schema identifier in query")
            }
            OperationError::UnknownFieldIdentifier => {
                write!(formatter, "Unknown field identifier in query")
            }
            OperationError::ExpressionTypeMismatch { left, right } => {
                write!(
                    formatter,
                    "Type mismatch in expression: {} and {} are not comparable",
                    left, right
                )
            }
            OperationError::InvalidExpressionType => {
                write!(formatter, "Invalid expression type for operation in query")
            }
        }
    }
}

impl Error for OperationError {}

impl Display for FieldType {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FieldType::Int => write!(formatter, "FieldType::Int"),
            FieldType::UInt => write!(formatter, "FieldType::UInt"),
            FieldType::Long => write!(formatter, "FieldType::Long"),
            FieldType::ULong => write!(formatter, "FieldType::ULong"),
            FieldType::Float => write!(formatter, "FieldType::Float"),
            FieldType::Bool => write!(formatter, "FieldType::Bool"),
            FieldType::DateTime => write!(formatter, "FieldType::DateTime"),
            FieldType::String => write!(formatter, "FieldType::String"),
            FieldType::ByteArray => write!(formatter, "FieldType::ByteArray"),
            FieldType::Array(a) => write!(formatter, "FieldType::Array({a})"),
            FieldType::Object(o) => write!(formatter, "FieldType::Object({})", o.id),
            FieldType::Enum(_) => write!(formatter, "FieldType::Enum"),
        }
    }
}

impl Debug for FieldType {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self)
    }
}
