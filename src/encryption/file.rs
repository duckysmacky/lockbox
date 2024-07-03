use std::{
    fs::{self, File},
    io::{Read, Result, Write},
    path::Path
};

pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let metadata = fs::metadata(path)?;
    let mut buffer = vec![0; metadata.len() as usize];

    file.read(&mut buffer)?;

    file.flush()?;
    Ok(buffer)
}

pub fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;

    file.write_all(bytes)?;

    file.flush()?;
    Ok(())
}