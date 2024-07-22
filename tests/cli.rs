use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ORIGINAL_DIR: &str = "test-files/original";
const TEST_DIR: &str = "test-files/test";

fn setup() {
    let test_path = Path::new(TEST_DIR);
    let original_path = Path::new(ORIGINAL_DIR);

    if !test_path.exists() {
        fs::create_dir_all(test_path).unwrap();
    }

    for entry in fs::read_dir(&original_path).unwrap() {
        let entry = entry.unwrap();
        let original_path = entry.path();
        if original_path.is_file() {
            let file_name = original_path.file_name().unwrap();
            let test_path = test_path.join(file_name);
            fs::copy(&original_path, &test_path).unwrap();
        }
    }
}

fn cleanup() {
    let test_path = Path::new(TEST_DIR);
    if test_path.exists() {
        fs::remove_dir_all(test_path).unwrap();
    }
}

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
fn test_encryption_decryption() {
    setup();

    let file_name = "word.txt";
    let test_file = Path::new(TEST_DIR).join(file_name);
    let original_file = Path::new(ORIGINAL_DIR).join(file_name);

    // Encrypt
    let output = encrypt(&test_file);
    assert!(output.status.success(), "Encryption failed: {:?}", output);

    // Decrypt
    let output = decrypt(&test_file);
    assert!(output.status.success(), "Decryption failed: {:?}", output);

    let original_content = fs::read_to_string(original_file).expect("failed to read original file");
    let decrypted_content = fs::read_to_string(test_file).expect("failed to read decrypted file");
    assert_eq!(original_content, decrypted_content, "Decrypted content doesn't match original content!");

    cleanup();
}