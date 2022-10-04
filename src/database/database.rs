use crate::archive::BlockFileReader;
use crate::schema::Schema;
use std::fs::File;
use std::io;

pub struct Database {
    pub reader: BlockFileReader,
    pub collections: Vec<Schema>,
}

impl Database {
    fn new(path: String, collections: Vec<Schema>) -> Result<Self, io::Error> {
        Ok(Self {
            reader: BlockFileReader::new(File::open(path)?),
            collections,
        })
    }
}
