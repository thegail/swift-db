use crate::archive::BlockFileIO;
use crate::schema::{Document, Schema};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::sync::mpsc::Receiver;

pub struct Backend {
    pub(super) io: BlockFileIO,
    pub(super) collections: Vec<Schema>,
    pub(super) document_cache: HashMap<usize, Document>,
    pub(super) reciever: Receiver<String>,
}

impl Backend {
    pub fn new(
        path: String,
        collections: Vec<Schema>,
        reciever: Receiver<String>,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            io: BlockFileIO::new(
                File::open(&path)?,
                OpenOptions::new().read(true).write(true).open(&path)?,
            ),
            collections,
            document_cache: HashMap::new(),
            reciever,
        })
    }
}
