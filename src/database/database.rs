use super::lifecycle_error::LifecycleError;
use crate::backend::Backend;
use crate::frontend::Connection;
use crate::schema::Schema;
use std::sync::mpsc::{channel, Sender};

struct Database {
    backend: Backend,
    connections: Vec<Connection>,
    sender: Sender<String>,
}

impl Database {
    fn new(path: String, collections: Vec<Schema>) -> Result<Self, LifecycleError> {
        let (sender, reciever) = channel();
        let db = Self {
            backend: Backend::new(path, collections, reciever)
                .map_err(LifecycleError::BackendError)?,
            connections: Vec::new(),
            sender,
        };
        Ok(db)
    }
}
