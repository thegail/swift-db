use crate::archive::ParseError;
use crate::schema::FieldType;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
enum UnderlyingOperationError {
    ParseError(ParseError),
    IOError(std::io::Error),
    UnknownSchemaIdentifier,
    ExpressionTypeMismatch(ExpressionTypeMismatch),
}

pub struct OperationError {
    underlying: UnderlyingOperationError,
}

impl OperationError {
    #![allow(non_snake_case)]
    pub fn ParseError(underlying: ParseError) -> Self {
        Self {
            underlying: UnderlyingOperationError::ParseError(underlying),
        }
    }

    pub fn IOError(underlying: std::io::Error) -> Self {
        Self {
            underlying: UnderlyingOperationError::IOError(underlying),
        }
    }

    pub fn UnknownSchemaIdentifier() -> Self {
        Self {
            underlying: UnderlyingOperationError::UnknownSchemaIdentifier,
        }
    }

    pub fn ExpressionTypeMismatch(left: FieldType, right: FieldType) -> Self {
        Self {
            underlying: UnderlyingOperationError::ExpressionTypeMismatch(ExpressionTypeMismatch {
                left,
                right,
            }),
        }
    }
}

impl Display for OperationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.underlying.fmt(formatter)
    }
}
impl Debug for OperationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.underlying.fmt(formatter)
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

struct ExpressionTypeMismatch {
    left: FieldType,
    right: FieldType,
}

impl Debug for ExpressionTypeMismatch {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "left: {}, right: {}", self.left, self.right)
    }
}
