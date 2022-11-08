use crate::schema::Document;

pub enum Response {
    Opened,
    Selected,
    Document(Document),
}

impl Response {
    pub fn serialize(self) -> String {
        match self {
            Response::Opened => "(ok opened)",
            Response::Selected => "(ok selected)",
            Response::Document(_) => "(ok placeholder)",
        }
        .to_string()
    }
}
