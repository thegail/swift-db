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
            Response::Opened => write!(out, "(ok open)")?,
            Response::Selected => write!(out, "(ok select)")?,
            Response::Document(doc) => {
                write!(out, "(ok read)")?;
                doc.transfer_serialize(out)?;
            }
        }
        Ok(())
    }
}
