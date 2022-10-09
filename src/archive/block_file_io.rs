use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom, Write};

pub struct BlockFileIO {
    reader: BufReader<File>,
    writer: File,
}

impl BlockFileIO {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file.try_clone().expect("File clone failed")),
            writer: file,
        }
    }

    pub fn next(&mut self) -> Result<Vec<u8>, Error> {
        loop {
            let mut buf = [0u8; 1];
            self.reader.read_exact(&mut buf)?;
            if buf[0] == 69 {
                break Ok(self.read_block()?);
            } else if buf[0] != 0 {
                break Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid byte"));
            }
        }
    }

    fn read_block(&mut self) -> Result<Vec<u8>, Error> {
        let mut length_bytes = [0u8; 4];
        self.reader.read_exact(&mut length_bytes)?;
        let block_length = ((length_bytes[0] as u64) << 48)
            + ((length_bytes[1] as u64) << 32)
            + ((length_bytes[2] as u64) << 16)
            + (length_bytes[3] as u64);

        let mut handle = self.reader.by_ref().take(block_length);
        let mut buffer: Vec<u8> = vec![];
        handle.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    fn read_at_position(&mut self, position: u64) -> Result<Vec<u8>, Error> {
        self.reader.seek(SeekFrom::Start(position))?;
        self.read_block()
    }

    pub fn reset_position(&mut self) -> Result<(), Error> {
        self.reader.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    pub fn write_block(&mut self, block: Vec<u8>) -> Result<(), Error> {
        let mut block = block;
        self.writer.seek(SeekFrom::End(0))?;
        let mut buf = vec![69u8];
        buf.extend_from_slice(&(buf.len() as u64).to_be_bytes());
        buf.append(&mut block);
        self.writer.write(&buf)?;
        Ok(())
    }
}
