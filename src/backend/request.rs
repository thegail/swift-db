use super::OperationError;
use super::{Query, Selection};
use crate::schema::Document;
use std::sync::mpsc::Sender;

pub struct Request {
    pub operation: Operation,
    pub return_channel: Sender<Result<Response, OperationError>>,
}

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
