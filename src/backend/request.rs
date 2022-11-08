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
    Create {
        document: Document,
    },
    Read {
        selection: Selection,
        fields: Vec<u16>,
    },
}

pub enum Response {
    Selection(Selection),
    Document(Document),
    Created,
}

impl Response {
    pub fn get_selection(self) -> Option<Selection> {
        match self {
            Response::Selection(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_document(self) -> Option<Document> {
        match self {
            Response::Document(d) => Some(d),
            _ => None,
        }
    }

    fn get_created(self) -> Option<()> {
        match self {
            Response::Created => Some(()),
            _ => None,
        }
    }
}
