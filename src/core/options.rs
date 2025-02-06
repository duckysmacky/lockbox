//! Provides structs which hold optional parameters for the API functions for easier option supply

use std::{collections::VecDeque, path::PathBuf};

/// Options for encryption
pub struct EncryptionOptions {
    /// Don't replace the name with a random UUID for the encrypted file
    pub keep_original_name: bool,
    /// Contains an output path for each file
    pub output_paths: Option<VecDeque<PathBuf>>
}

impl Default for EncryptionOptions {
    fn default() -> Self {
        EncryptionOptions {
            keep_original_name: false,
            output_paths: None,
        }
    }
}

/// Options for decryption
pub struct DecryptionOptions {
    /// Contains an output path for each file
    pub output_paths: Option<VecDeque<PathBuf>>
}

impl Default for DecryptionOptions {
    fn default() -> Self {
        DecryptionOptions {
            output_paths: None,
        }
    }
}

/// Options for key retrieval
pub struct KeyGetOptions {
    /// Format encryption key as list of bytes
    pub as_byte_array: bool
}

impl Default for KeyGetOptions {
    fn default() -> Self {
        KeyGetOptions {
            as_byte_array: false,
        }
    }
}