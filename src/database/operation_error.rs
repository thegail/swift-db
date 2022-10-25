use crate::archive::ParseError;
use crate::schema::FieldType;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum OperationError {
    ParseError(ParseError),
    IOError(std::io::Error),
    UnknownSchemaIdentifier,
    ExpressionTypeMismatch(ExpressionTypeMismatch),
}

impl OperationError {
    pub fn expression_type_mismatch(left: FieldType, right: FieldType) -> Self {
        Self::ExpressionTypeMismatch(ExpressionTypeMismatch { left, right })
    }
}

impl Display for OperationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self)
    }
}
impl Error for OperationError {}

struct ExpressionTypeMismatch {
    left: FieldType,
    right: FieldType,
}

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

impl Debug for ExpressionTypeMismatch {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "left: {}, right: {}", self.left, self.right);
        Ok(())
    }
}
