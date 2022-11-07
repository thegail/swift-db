use super::{Query, Selection};
use crate::schema::Document;

pub enum Operation {
    FindOne {
        query: Query,
    },
    Read {
        selection: Selection,
        fields: Vec<u16>,
    },
}

pub enum Response {
    Selection(Selection),
    Document(Document),
}
