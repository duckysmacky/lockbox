use std::{
    io::{self, Read, Write},
    fs::{self, File},
    os::windows::fs::MetadataExt,
    ffi::OsString,
    path::Path
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use super::cipher::{self, Key, Nonce};

pub type Checksum = [u8; 32];

mod box_data {
    pub const VERSION: u8 = 1;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxFile {
    header: BoxHeader,
    body: Vec<u8>
}

pub fn parse_file(path: &Path, key: Key) -> io::Result<(BoxHeader, Vec<u8>)> {
    let mut file = File::open(path)?;
    let metadata = fs::metadata(path)?;

    let mut buffer = vec![0; metadata.len() as usize];

    file.read(&mut buffer)?;
    
    let box_file: BoxFile = bincode::deserialize(&buffer).expect("Failed to deserialize box file");

    let header = box_file.header;
    let body = box_file.body;

    let nonce = header.nonce;
    let body = cipher::decrypt(&key, &nonce, &body).expect("Failed to decrypt body data");

    file.flush()?;
    Ok((header, body))
}

pub fn write_file(path: &Path, header: BoxHeader, body: Vec<u8>) -> io::Result<()> {
    let mut file = File::create(path)?;

    let box_file = BoxFile {header, body};
    let box_data = bincode::serialize(&box_file).expect("Failed to serialize box file");

    file.write_all(&box_data)?;

    file.flush()?;
    Ok(())
}

pub fn generate_header(path: &Path, checksum: Checksum, nonce: Nonce) -> BoxHeader {
    let file_data = fs::metadata(path).expect("Unable to get file metadata");

    let mut header = BoxHeader {
        magic: box_data::MAGIC,
        version: box_data::VERSION,
        metadata_length: 0,
        original_filename: match path.file_stem() {
            None => OsString::from(""),
            Some(file_stem) => file_stem.to_os_string()
        },
        original_extension: match path.extension() {
            None => OsString::from(""),
            Some(extension) => extension.to_os_string()
        },
        original_size: file_data.file_size(),
        checksum,
        nonce
    };

    header.metadata_length = get_metadata_len(&header);
    header
}

pub fn get_header(path: &Path) -> io::Result<BoxHeader> {
    let mut file = File::open(path)?;
    let metadata = fs::metadata(path)?;
    let mut buffer = vec![0; metadata.len() as usize];

    file.read(&mut buffer)?;
    let box_file: BoxFile = bincode::deserialize(&buffer).expect("Failed to deserialize box file");
    let header = box_file.header;

    Ok(header)
}

pub fn generate_checksum(data: &[u8]) -> Checksum {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();
    let mut checksum = [0u8; 32];

    checksum.copy_from_slice(&result);

    checksum
}

// TODO - optimise
fn get_metadata_len(header: &BoxHeader) -> u16 {
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