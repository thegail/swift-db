use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DeserializationError {
    FieldNotFound(String),
    FieldTypeMismatch(String),
    Overflow(i64),
}

impl Display for DeserializationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            DeserializationError::FieldNotFound(s) => write!(formatter, "Field {} not found", s),
            DeserializationError::FieldTypeMismatch(f) => {
                write!(formatter, "Incorrect type in field {}", f)
            }
            DeserializationError::Overflow(v) => write!(formatter, "Value {} overflows field", v),
        }
    }
}
