use crate::Key;
use crate::error::{Result, Error};

/// transforms Key type into hex string
/// (e.g. \[1, 40, 174, 16, 5, ...\] into "0128AE1005...")
pub fn key_to_hex_string(key: Key) -> String {
    let mut res: String = String::new();
    for elem in key {
        if elem < 0x10 { res.push_str(&format!("0{:X}", elem)) }
        else { res.push_str(&format!("{:X}", elem)) }
    }
    return res;
}

/// transforms hex string into Key type
/// (e.g. "0128AE1005..." into \[1, 40, 174, 16, 5, ...\])
pub fn hex_string_to_key(hex: String) -> Result<Key> {
    let mut res: Key = [0; 32];
    let safe_chars: &str = "0123456789ABCDEF";
    if hex.len() != 64 {
        return Err(Error::InvalidInput("Hex string has invalid length".to_string()));
    }
    let mut i: usize = 0;
    while i < hex.len() {
        // we can just unwrap this since the length is guaranteed to be fine
        let c1: char = hex.chars().nth(i).unwrap();
        let c2: char = hex.chars().nth(i + 1).unwrap();
        if !safe_chars.contains(c1) || !safe_chars.contains(c2) {
            return Err(Error::InvalidInput(format!("Invalid byte '{}{}' in hex string", c1, c2)));
        }
        // we can just unwrap this too since we sanity-checked the characters beforehand
        res[i / 2] = u8::from_str_radix(&format!("{}{}", c1, c2), 16).unwrap();

        i += 2;
    }
    Ok(res)
}