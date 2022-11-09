use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DeserializationError {
    FieldNotFound(String),
}

impl Display for DeserializationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            DeserializationError::FieldNotFound(s) => write!(formatter, "Field {} not found", s),
        }
    }
}
