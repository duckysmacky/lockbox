//! Contains functions for parsing file with custom file format, as well as reading header
//! information

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use crate::{Result, Error};
use super::{BoxFile, BoxHeader};

/// Reads the `.box` file at the provided file and returns parsed information
pub fn parse_file(path: &Path) -> Result<BoxFile> {
    let mut file = File::open(path)?;
    let metadata = fs::metadata(path)?;

    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;

    let box_file: BoxFile = bincode::deserialize(&buffer)
        .map_err(|err| Error::InvalidData(format!("Unable to deserialize \"{}\" file data for: {}", path.display(), err)))?;
    Ok(box_file)
}

/// Writes header information and bytes to the file at the provided path
pub fn write_file(path: &Path, header: BoxHeader, body: Vec<u8>) -> Result<()> {
    let mut file = File::create(path)?;

    let box_file = BoxFile {header, body};
    let box_data = bincode::serialize(&box_file)
        .map_err(|err| Error::InvalidData(format!("Unable to serialize \"{}\" file data for: {}", path.display(), err)))?;

    file.write_all(&box_data)?;

    file.flush()?;
    Ok(())
}