use crate::schema::Document;
use serde_json::to_writer;
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
                to_writer(out.by_ref(), &doc)?;
                writeln!(out)?;
            }
        }
        Ok(())
    }
}
