use crate::schema::Document;

pub enum Response {
    Opened,
    Selected,
    Document(Document),
}

impl Response {
    pub fn serialize(self) -> String {
        todo!()
    }
}
