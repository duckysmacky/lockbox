use std::{
    fs::{self, File},
    io::{Error, Read, Write},
    path::Path
};

pub fn read_bytes(path: &Path) -> Result<Vec<u8>, Error> {
    let metadata = fs::metadata(path)?;
    let mut file = File::open(path)?;
    let mut buffer = vec![0; metadata.len() as usize];

    file.read(&mut buffer)?;

    file.flush()?;
    Ok(buffer)
}

pub fn write_bytes(path: &Path, bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(bytes)?;
    file.flush()?;
    Ok(Vec::from(bytes))
}