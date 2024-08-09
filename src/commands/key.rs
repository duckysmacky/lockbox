use std::io;
use crate::log_success;
use crate::storage::keys;

pub struct NewOptions {
    // TODO
}

pub struct DeleteOptions {
    // TODO
}

pub fn new_key(_options: &NewOptions) -> io::Result<()> {
    log_success!("Generating a new encryption key");
    keys::generate_new_key();
    Ok(())
}

pub fn delete_key(_options: &DeleteOptions) -> io::Result<()> {
    log_success!("Deleting encryption key");
    keys::delete_key();
    Ok(())
}