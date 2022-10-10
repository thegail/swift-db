use crate::archive::BlockFileIO;
use crate::schema::Schema;
use std::fs::{File, OpenOptions};
use std::io;

pub struct Database {
    pub io: BlockFileIO,
    pub collections: Vec<Schema>,
}

impl Database {
    pub fn new(path: String, collections: Vec<Schema>) -> Result<Self, io::Error> {
        Ok(Self {
            io: BlockFileIO::new(
                File::open(&path)?,
                OpenOptions::new().read(true).write(true).open(&path)?,
            ),
            collections,
        })
    }
}
