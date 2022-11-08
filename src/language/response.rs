use crate::schema::Document;

pub enum Response {
    Selected,
    Document(Document),
}
