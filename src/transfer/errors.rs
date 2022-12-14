use std::fmt::{Display, Formatter};

/// An error while serializing or deserializing a [`Document`].
/// When thrown, this error is wrapped in a [`ParseError`] by
/// the language parser.
///
/// [`Document`]: crate::schema::Document
/// [`ParseError`]: crate::language::ParseError
#[derive(Debug)]
pub enum DeserializationError {
    FieldNotFound(String),
    CaseNotFound(String),
    FieldTypeMismatch,
    Overflow(i64),
    ParseError(serde_json::Error),
}

impl Display for DeserializationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            DeserializationError::FieldNotFound(s) => write!(formatter, "Field {} not found", s),
            DeserializationError::CaseNotFound(s) => write!(formatter, "Case {} not found", s),
            DeserializationError::FieldTypeMismatch => write!(formatter, "Incorrect type in field"),
            DeserializationError::Overflow(v) => write!(formatter, "Value {} overflows field", v),
            DeserializationError::ParseError(e) => write!(formatter, "JSON parse error: {}", e),
        }
    }
}
// TODO change name
