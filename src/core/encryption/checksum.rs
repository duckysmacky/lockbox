//! Contains functions related to checksum

use sha2::{Digest, Sha256};
use crate::Checksum;

/// Generates checksum for data validation from given byte slice
pub fn generate_checksum(data: &[u8]) -> Checksum {
    let mut hasher = Sha256::new();
    hasher.update(data);

    let result = hasher.finalize();
    let mut checksum = [0u8; 32];

    checksum.copy_from_slice(&result);
    checksum
}