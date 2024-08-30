use std::{ffi::OsString, fs, path::Path};
use crate::core::encryption::checksum;
use crate::{Nonce, Result};
use super::BoxHeader;

pub fn generate_header(path: &Path, data: &[u8], nonce: &Nonce) -> Result<BoxHeader> {
    let file_data = fs::metadata(path)?;

    let mut header = BoxHeader {
        magic: super::header_data::MAGIC,
        version: super::header_data::VERSION,
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