use crate::backend::{Backend, Request};
use crate::database::LifecycleError;
use crate::frontend::Connection;
use crate::schema::Schema;
use std::net::TcpListener;
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;

/// Implements SwiftDB's main logic.
///
/// On creation, a `Database` loads its configuration and creates
/// a [`Backend`]. When [`start()`][Database::start()] is called,
/// it creates a new thread which the backend listens on, then
/// starts a TCP listener, creating a thread with a [`Connection`]
/// for each incoming connection.
pub struct Database {
    backend: Backend,
    sender: Sender<Request>,
    collections: Vec<Schema>,
}

impl Database {
    /// Creates a [`Database`] instance.
    ///
    /// Loads configuration and creates a [`Backend`], along with
    /// an MPSC channel for communication between frontends and
    /// the backend.
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

    /// Starts SwiftDB's main loop.
    ///
    /// Starts the backend's command listener on the backend thread,
    /// then a TCP listener on the main thread. Each incoming
    /// connection is passed off to a [`Connection`] on a new thread.
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
