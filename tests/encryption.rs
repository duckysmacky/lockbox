mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::*;

fn encrypt(path: &PathBuf) -> Output {
    Command::new("lockbox")
        .args(&["box", path.to_str().unwrap()])
        .output()
        .expect("failed to execute process")
}

fn decrypt(path: &PathBuf) -> Output {
    Command::new("lockbox")
        .args(&["unbox", path.to_str().unwrap()])
        .output()
        .expect("failed to execute process")
}

#[test]
fn test_word_encryption() {
    let file_name = "word.txt";
    setup();

    let test_file = Path::new(TEST_DIR).join(file_name);
    let original_file = Path::new(ORIGINAL_DIR).join(file_name);

    let output = encrypt(&test_file);
    assert!(output.status.success(), "Encryption failed: {:?}", output);

    let output = decrypt(&test_file);
    assert!(output.status.success(), "Decryption failed: {:?}", output);

    let original_content = fs::read_to_string(original_file).expect("failed to read original file");
    let decrypted_content = fs::read_to_string(test_file).expect("failed to read decrypted file");
    assert_eq!(original_content, decrypted_content, "Decrypted content doesn't match original content!");

    cleanup();
}

#[test]
fn test_words_encryption() {
    let file_name = "words.txt";
    setup();

    let test_file = Path::new(TEST_DIR).join(file_name);
    let original_file = Path::new(ORIGINAL_DIR).join(file_name);

    let output = encrypt(&test_file);
    assert!(output.status.success(), "Encryption failed: {:?}", output);

    let output = decrypt(&test_file);
    assert!(output.status.success(), "Decryption failed: {:?}", output);

    let original_content = fs::read_to_string(original_file).expect("failed to read original file");
    let decrypted_content = fs::read_to_string(test_file).expect("failed to read decrypted file");
    assert_eq!(original_content, decrypted_content, "Decrypted content doesn't match original content!");

    cleanup();
}