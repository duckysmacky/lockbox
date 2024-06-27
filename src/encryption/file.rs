use std::{
    fs::{self, File},
    io::{Error, Read, Write}
};

pub fn read_bytes(path: &str) -> Result<Vec<u8>, Error> {
    let metadata = fs::metadata(path).expect("Unable to read metadata");
    let mut file = File::open(path).expect("Specified file doesnt exist");
    let mut buffer = vec![0; metadata.len() as usize];

    file.read(&mut buffer).expect("Buffer overflow");

    file.flush()?;
    Ok(buffer)
}

pub fn write_bytes(path: &str, bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Specified file doesnt exist");

    file.write_all(bytes)?;
    file.flush()?;
    Ok(Vec::from(bytes))
}