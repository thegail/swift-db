use crate::schema::Document;
use std::io::Write;

pub enum Response {
    Opened,
    Selected,
    Document(Document),
}

impl Response {
    pub fn serialize(self, mut out: impl Write) -> Result<(), std::io::Error> {
        match self {
            Response::Opened => writeln!(out, "(ok opened)")?,
            Response::Selected => writeln!(out, "(ok selected)")?,
            Response::Document(doc) => {
                let write_result = doc.into_writer(out.by_ref());
                if let Err(error) = write_result {
                    writeln!(out, "Serialization error: {}", error)?;
                }
                writeln!(out)?;
            }
        }
        Ok(())
    }
}
