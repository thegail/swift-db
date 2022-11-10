//! The `archive` module parses and produces the archive
//! binary serialization format used to store data.
//!
//! Its utilities are used by the [`backend`][crate::backend].
mod archive_parser;
mod block_file_io;
mod document_serialize;
mod parse_error;

pub use archive_parser::ArchiveParser;
pub use block_file_io::BlockFileIO;
pub use parse_error::ParseError;
