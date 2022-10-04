use crate::archive::BlockFileReader;
use std::fs::File;
use std::io;

pub struct Database {
    reader: BlockFileReader,
}

impl Database {
    fn new(path: String) -> Result<Self, io::Error> {
        Ok(Self {
            reader: BlockFileReader::new(File::open(path)?),
        })
    }
}
