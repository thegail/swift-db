use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom, Write};

/// A utility to read and write blocks of data to/from a storage file.
///
/// An instance of a `BlockFileIO` manager is owned by the
/// [`Backend`][crate::backend::Backend], which calls
/// [`next`][BlockFileIO#method.next],
/// [`read_at_position`][BlockFileIO#method.read_at_position],
/// and [`write_block`][BlockFileIO#method.write] to read and write
/// [`Document`][crate::schema::Document]s.
pub struct BlockFileIO {
    reader: BufReader<File>,
    writer: File,
}

impl BlockFileIO {
    /// Creates a new [`BlockFileIO`] manager.
    ///
    /// Accepts two File instances, both pointing to the database's
    /// storage file.
    pub fn new(read_file: File, write_file: File) -> Self {
        Self {
            reader: BufReader::new(read_file),
            writer: write_file,
        }
    }

    /// Reads the next block in the file.
    ///
    /// Returns the position of the block and the block data.
    pub fn next(&mut self) -> Result<(usize, Vec<u8>), Error> {
        loop {
            let mut buf = [0u8; 1];
            self.reader.read_exact(&mut buf)?;
            if buf[0] == 69 {
                break Ok((self.reader.stream_position()? as usize, self.read_block()?));
            } else if buf[0] != 0 {
                break Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid byte"));
            }
        }
    }

    fn read_block(&mut self) -> Result<Vec<u8>, Error> {
        let mut length_bytes = [0u8; 8];
        self.reader.read_exact(&mut length_bytes)?;
        let block_length = u64::from_be_bytes(length_bytes);

        let mut handle = self.reader.by_ref().take(block_length);
        let mut buffer: Vec<u8> = vec![];
        handle.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    /// Read the data of a block at a certain position.
    pub fn read_at_position(&mut self, position: u64) -> Result<Vec<u8>, Error> {
        self.reader.seek(SeekFrom::Start(position))?;
        self.read_block()
    }

    /// Seek to the beginning of the file.
    pub fn reset_position(&mut self) -> Result<(), Error> {
        self.reader.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    /// Write a block, appending it to the end of the file.
    ///
    /// Returns the position at which the block was written.
    pub fn write_block(&mut self, block: Vec<u8>) -> Result<usize, Error> {
        let mut block = block;
        self.writer.seek(SeekFrom::End(0))?;
        let position = self.writer.stream_position()?;
        let mut buf = vec![69u8];
        buf.extend_from_slice(&(block.len() as u64).to_be_bytes());
        buf.append(&mut block);
        self.writer.write_all(&buf)?;
        Ok(position as usize + 1)
    }
}
