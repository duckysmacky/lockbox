//! Contains functions related to manipulating header for the custom file format (.box)

use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::ffi::OsString;
use crate::core::encryption::checksum;
use crate::{new_err, Checksum, Nonce, Result};
use header_info::{VERSION, MAGIC};

pub mod header_info {
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

/// Generates and returns a new header for the file at the provided path based on its data, as well
/// as generates checksum for the provided data
pub fn generate_header(path: &Path, data: &[u8], nonce: &Nonce) -> Result<BoxHeader> {
    let file_data = fs::metadata(path)?;

    let mut header = BoxHeader {
        magic: MAGIC,
        version: VERSION,
        metadata_length: 0,
        original_filename: match path.file_stem() {
            None => OsString::from(""),
            Some(file_stem) => file_stem.to_os_string()
        },
        original_extension: match path.extension() {
            None => OsString::from(""),
            Some(extension) => extension.to_os_string()
        },
        original_size: file_data.len(),
        checksum: checksum::generate_checksum(data),
        nonce: *nonce
    };

    header.metadata_length = get_metadata_length(&header);
    Ok(header)
}

/// Reads the `.box` file at the provided file and returns parsed information
pub fn parse_file(path: &Path) -> Result<BoxFile> {
    let mut file = File::open(path)?;
    let metadata = fs::metadata(path)?;

    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;

    let box_file: BoxFile = bincode::deserialize(&buffer)
        .map_err(|err| new_err!(SerializeError: BOXParseError, err))?;
    Ok(box_file)
}

/// Writes header information and bytes to the file at the provided path
pub fn write_file(path: &Path, header: BoxHeader, body: Vec<u8>) -> Result<()> {
    let mut file = File::create(path)?;

    let box_file = BoxFile {header, body};
    let box_data = bincode::serialize(&box_file)
        .map_err(|err| new_err!(SerializeError: BOXParseError, err))?;

    file.write_all(&box_data)?;

    file.flush()?;
    Ok(())
}

fn get_metadata_length(header: &BoxHeader) -> u16 {
    let mut metadata_len = 0;

    metadata_len += 4; // magic: u8 * 4 = 4 bytes
    metadata_len += 1; // version: u8 = 1 byte
    metadata_len += 2; // metadata_len: u16 = 2 bytes
    metadata_len += header.original_filename.len();
    metadata_len += header.original_extension.len();
    metadata_len += 8; // original_size: u64 = 8 bytes
    metadata_len += 32; // checksum: u8 * 32 = 32 bytes
    metadata_len += 12; // nonce: u8 * 12 = 12 bytes

    metadata_len as u16
}