use super::lifecycle_error::LifecycleError;
use crate::backend::{Backend, Request};
use crate::frontend::Connection;
use crate::schema::Schema;
use std::net::TcpListener;
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

pub struct Database {
    backend: Backend,
    sender: Sender<Request>,
}

impl Database {
    pub fn new(path: String, collections: Vec<Schema>) -> Result<Self, LifecycleError> {
        let (sender, reciever) = channel();
        let db = Self {
            backend: Backend::new(path, collections, reciever)
                .map_err(LifecycleError::BackendError)?,
            sender,
        };
        Ok(db)
    }

    pub fn start(mut self) -> Result<(), LifecycleError> {
        spawn(move || {
            self.backend.listen();
        });
        let listener = TcpListener::bind("localhost:1952").map_err(LifecycleError::NetworkError)?;
        for connection in listener.incoming() {
            match connection {
                Ok(stream) => {
                    let mut connection = Connection::new(stream, self.sender.clone());
                    spawn(move || connection.listen());
                }
                Err(_) => (),
            }
        }
        Ok(())
    }
}
