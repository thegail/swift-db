use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DeserializationError {}

impl Display for DeserializationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {}
    }
}
