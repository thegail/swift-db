use crate::schema::Document;

pub enum Response {
    Opened,
    Selected,
    Document(Document),
}
