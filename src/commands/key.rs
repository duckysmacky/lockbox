use std::io;
use crate::storage::keys;

pub struct NewOptions {
    // TODO
}

pub struct DeleteOptions {
    // TODO
}

pub fn new_key(_options: &NewOptions) -> io::Result<()> {
    keys::generate_new_key();
    Ok(())
}

pub fn delete_key(_options: &DeleteOptions) -> io::Result<()> {
    keys::delete_key();
    Ok(())
}