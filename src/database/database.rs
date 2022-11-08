use crate::backend::{Backend, Request};
use crate::database::LifecycleError;
use crate::frontend::Connection;
use crate::schema::Schema;
use std::net::TcpListener;
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

pub struct Database {
    backend: Backend,
    sender: Sender<Request>,
    collections: Vec<Schema>,
}

impl Database {
    pub fn new(path: String, collections: Vec<Schema>) -> Result<Self, LifecycleError> {
        let (sender, reciever) = channel();
        let db = Self {
            backend: Backend::new(path, collections.clone(), reciever)
                .map_err(LifecycleError::BackendError)?,
            sender,
            collections,
        };
        Ok(db)
    }

    pub fn start(mut self) -> Result<(), LifecycleError> {
        spawn(move || {
            self.backend.listen();
        });
        let listener = TcpListener::bind("localhost:1952").map_err(LifecycleError::NetworkError)?;
        for stream in listener.incoming().flatten() {
            let mut connection =
                Connection::new(stream, self.sender.clone(), self.collections.clone());
            spawn(move || connection.listen());
        }
        Ok(())
    }
}
