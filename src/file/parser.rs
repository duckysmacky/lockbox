use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};
use std::path::Path;
use crate::file::{BoxFile, BoxHeader};

pub fn parse_file(path: &Path) -> io::Result<BoxFile> {
    let mut file = File::open(path)?;
    let metadata = fs::metadata(path)?;

    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;

    let box_file: BoxFile = bincode::deserialize(&buffer)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("Unable to deserialize file data: {}", err)))?;
    Ok(box_file)
}

pub fn write_file(path: &Path, header: BoxHeader, body: Vec<u8>) -> io::Result<()> {
    let mut file = File::create(path)?;

    let box_file = BoxFile {header, body};
    let box_data = bincode::serialize(&box_file)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("Unable to serialize file data: {}", err)))?;

    file.write_all(&box_data)?;

    file.flush()?;
    Ok(())
}