//! Contains implementation for the custom `boxfile` file format, it's header and
//! additional information for parsing and serializing the custom file format.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use crate::{new_err, Checksum, Key, Nonce, Result};
use crate::core::data::io;
use super::cipher;

mod header_info {
    //! Constants for the header: current file format version and unique file
    //! identifier (magic)
    /// Version of the `boxfile` format being used for backwards compatibility
    pub const VERSION: u8 = 1;
    /// Unique identifier for the `boxfile` file format
    pub const MAGIC: [u8; 4] = [b'B', b'O', b'X', VERSION];
}

/// Struct representing a `boxfile` structure. A "boxfile" is the custom file 
/// format for lockbox which contains the encrypted data of a file, alongside
/// header with extra information and random padding. It is generated as a result
/// of file encryption operation and has a `.box` extension.
///
/// *The `boxfile` structure is heavily inspired by the SSH Packet structure. As it
/// is known to be safe and efficient*
#[derive(Serialize, Deserialize)]
pub struct Boxfile {
    /// Custom header for the boxfile. Not encrypted unlike the body of the file and
    /// is available for reading by other processing, meaning an encryption key is
    /// not required, as doesn't contain any sensetive information.
    ///
    /// *Could be a subject to change in the future*
    header: BoxfileHeader,
    /// File body is the encrypted content of the original file together with randomly
    /// generated `padding`. It is the main payload for the entire `boxfile`. Can be
    /// compressed for reduced storage size
    ///
    /// `Padding` is a randomly generated array of random bytes used for encryption
    /// obfuscation. It is encrypted together with the original file so that bytes
    /// mix together and make information even more unreadable without a decryption
    /// key. There must be at least 4 bytes of padding and a maximum of 255 bytes.
    ///
    /// Padding should be of such a length, that the total length of any `boxfile`
    /// component (header/body/padding itself) is a multiple of the cipher block
    /// size or 8 (whichever is larger).
    body: Box<[u8]>,
    /// Checksum is a hash generated from the content of the `boxfile` file body
    /// before the encryption occurs. It ensures the data's integrity by comparing
    /// it to the checksum generated after decryption of the same file.
    checksum: Checksum
}

impl Boxfile {
    /// Generates a new `boxfile` from the provided file. Creates a new `BoxfileHeader`
    /// and stores original file's name and extension in it, also generates a unique 
    /// `Nonce` for later usage in encryption. Padding is also generated during this
    /// step and added at the end of the original file's data as a part of the body.
    /// Checksum is generated at the very end from the header and body content.
    pub fn new(file_path: PathBuf) -> Result<Self> {
        let file_data = io::read_bytes(&file_path)?;
        let padding_len: u8 = (file_data.len() as u8 / 8) + 1;
        let padding = Self::generate_padding(padding_len);
        let body = [file_data, padding].concat();

        let header = BoxfileHeader::new(
            file_path.file_stem(),
            file_path.extension(),
            padding_len,
            cipher::generate_nonce()
        );

        let mut hasher = Sha256::new();
        hasher.update(&header.as_bytes()?);
        hasher.update(&body);
        let result = hasher.finalize();
        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&result);
        
        Ok(Self {
            header,
            body: body.into(),
            checksum
        })
    }

    /// Parses the provided file, tries to deserialize it and returns a parsed `boxfile`
    pub fn parse(file_path: &Path) -> Result<Self> {
        let bytes = io::read_bytes(file_path)?;
        let boxfile: Boxfile = bincode::deserialize(&bytes)
            .map_err(|err| new_err!(SerializeError: BoxfileParseError, err))?;

        Ok(boxfile)
    }

    /// Returns the information about the file contained within the `boxfile`: original
    /// file name and extension.
    pub fn file_info(&self) -> (&OsString, &OsString) {
        let file_name = &self.header.original_name;
        let file_extension = &self.header.original_extension;
        (file_name, file_extension)
    }

    /// Verifies checksum for the `boxfile` by generating new checksum for current data and
    /// comparing it to the checksum stored in the header
    pub fn verify_checksum(&self) -> Result<bool> {
        let mut hasher = Sha256::new();
        hasher.update(&self.header.as_bytes()?);
        hasher.update(&self.body);

        let result = hasher.finalize();
        let mut checksum = [0u8; 32];

        checksum.copy_from_slice(&result);
        Ok(checksum == self.checksum)
    }

    /// Serializes self and writes to specified file
    pub fn save_to(&self, path: &Path) -> Result<()> {
        let bytes = bincode::serialize(&self)
            .map_err(|err| new_err!(SerializeError: BoxfileParseError, err))?;
        io::write_bytes(path, &bytes, true)?;

        Ok(())
    }

    /// Encrypts the body of the `boxfile` together with randomly generated padding 
    /// using the provided encryption key
    pub fn encrypt_data(&mut self, key: &Key) -> Result<()> {
        let encrypted_body = cipher::encrypt(key, &self.header.nonce, &self.body)?;
        self.body = encrypted_body.into();
        Ok(())
    }
    
    /// Decrypts the body of the `boxfile` and removes the unneeded padding, returning
    /// only the actual data content of the original file
    pub fn decrypt_data(&mut self, key: &Key) -> Result<Box<[u8]>> {
        let decrypted_body = cipher::decrypt(key, &self.header.nonce, &self.body)?;
        let padding_len = self.header.padding_len;
        let data_len = decrypted_body.len() as i32 - padding_len as i32;
        if data_len < 0 {
            return Err(new_err!(SerializeError: BoxfileParseError, "Invalid file data length"))
        }
        let file_data = &decrypted_body[..data_len as usize];
        Ok(file_data.into())
    }

    // TODO: generate padding based on the length of the cipher block size and data
    /// Generates random padding of specified length
    fn generate_padding(padding_len: u8) -> Vec<u8> {
        vec![0u8; padding_len as usize]
    }
}

/// The header for the `boxfile`, which contains extra information about the file. This
/// includes a unique identifier (magic), length of the generated padding, the original
/// file name and extension and generated `Nonce` for encryption/decryption uniqueness
#[derive(Serialize, Deserialize)]
struct BoxfileHeader {
    /// Unique identifier for the file format including the used version
    magic: [u8; 4],
    /// The length of the generated padding
    padding_len: u8,
    /// The original name of the file
    original_name: OsString,
    /// The original extension of the file
    original_extension: OsString,
    /// Randomly generated 12-byte `Nonce` used for encryption and decryption. Ensures
    /// that no ciphertext generated using one key is the same
    nonce: Nonce
}

impl BoxfileHeader {
    pub fn new(
        file_name: Option<&OsStr>,
        file_extension: Option<&OsStr>,
        padding_len: u8,
        nonce: Nonce 
    ) -> Self {
        let file_name = match file_name {
            None => OsString::from("unknown"),
            Some(name) => OsString::from(name)
        };
        let file_extension = match file_extension {
            None => OsString::from(""),
            Some(name) => OsString::from(name)
        };

        BoxfileHeader {
            magic: header_info::MAGIC,
            original_name: file_name,
            original_extension: file_extension,
            padding_len,
            nonce
        }
    }

    /// Returns the header serialized as plain bytes
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let bytes = bincode::serialize(&self)
            .map_err(|err| new_err!(SerializeError: HeaderParseError, err))?;
        Ok(bytes)
    }
}

