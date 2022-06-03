use std::fs::File;
use std::io::{BufReader, Read};

pub struct BlockFileReader {
    reader: BufReader<File>,
}

impl BlockFileReader {
    fn next(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let first_byte = loop {
            let mut buf = [0u8; 1];
            self.reader.read_exact(&mut buf)?;
            if buf[0] != 0 {
                break buf[0];
            }
        };
        let mut other_bytes = [0u8; 3];
        self.reader.read_exact(&mut other_bytes)?;
        let block_length = ((first_byte as u64) << 48)
            + ((other_bytes[0] as u64) << 32)
            + ((other_bytes[1] as u64) << 16)
            + (other_bytes[2] as u64);

        let mut handle = self.reader.by_ref().take(block_length);
        let mut buffer: Vec<u8> = vec![];
        handle.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}
