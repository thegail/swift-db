use crate::schema::Document;
use std::io::Write;

pub enum Response {
    Opened,
    Selected,
    Document(Document),
}

impl Response {
    pub fn serialize(self, out: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Response::Opened => writeln!(out, "(ok opened)")?,
            Response::Selected => writeln!(out, "(ok selected)")?,
            Response::Document(doc) => {
                writeln!(out, "(ok document)")?;
                doc.transfer_serialize(out)?;
                writeln!(out)?;
            }
        }
        Ok(())
    }
}
