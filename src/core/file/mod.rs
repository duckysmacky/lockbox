//! Contains functions for the custom file format parsing and general IO operations

use std::ffi::OsString;
use serde::{Deserialize, Serialize};
use crate::{Nonce, Checksum};

pub mod io;
pub mod parser;
pub mod header;

pub mod header_data {
    /// Version of the file format being used for backwards compatibility
    pub const VERSION: u8 = 1;
    /// Unique identifier for the file format
    pub const MAGIC: [u8; 4] = [b'B', b'O', b'X', VERSION];
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxHeader {
    magic: [u8; 4],
    version: u8,
    metadata_length: u16,
    pub original_filename: OsString,
    pub original_extension: OsString,
    pub original_size: u64,
    pub checksum: Checksum,
    pub nonce: Nonce
}

/// `.box` file format structure
#[derive(Serialize, Deserialize, Debug)]
pub struct BoxFile {
    pub header: BoxHeader,
    pub body: Vec<u8>
}