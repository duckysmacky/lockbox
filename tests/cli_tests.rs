use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use common::*;

mod common;

fn encrypt_command(path: &PathBuf) -> Output {
    Command::new("lockbox")
        .args(&["-v", "-p", PASSWORD, "box", path.to_str().unwrap()])
        .output()
        .expect("Failed to execute the \"box\" command")
}

fn decrypt_command(path: &PathBuf) -> Output {
    Command::new("lockbox")
        .args(&["-v", "-p", PASSWORD, "unbox", path.to_str().unwrap()])
        .output()
        .expect("Failed to execute the \"unbox\" command")
}

fn format_output(text: &str, output: Output) -> String {
    format!("{}\nSTDOUT: {:?}\nSTDERR: {:?}\nSTATUS CODE: {}\n", text, output.stdout, output.stderr, output.status)
}

#[test]
fn test_word_encryption() {
    let file_name = "word.txt";
    setup();

    let test_file = Path::new(TEST_DIR).join(file_name);
    let original_file = Path::new(ORIGINAL_DIR).join(file_name);

    let output = encrypt_command(&test_file);
    assert!(output.status.success(), "{}", format_output("Encryption failed", output));

    let output = decrypt_command(&test_file);
    assert!(output.status.success(), "{}", format_output("Decryption failed", output));

    let original_content = fs::read_to_string(original_file).expect("Failed to read original file");
    let decrypted_content = fs::read_to_string(test_file).expect("Failed to read decrypted file");
    assert_eq!(original_content, decrypted_content, "Decrypted content doesn't match original content");

    cleanup();
}

#[test]
fn test_words_encryption() {
    let file_name = "words.txt";
    setup();

    let test_file = Path::new(TEST_DIR).join(file_name);
    let original_file = Path::new(ORIGINAL_DIR).join(file_name);

    let output = encrypt_command(&test_file);
    assert!(output.status.success(), "{}", format_output("Encryption failed", output));

    let output = decrypt_command(&test_file);
    assert!(output.status.success(), "{}", format_output("Decryption failed", output));

    let original_content = fs::read_to_string(original_file).expect("Failed to read original file");
    let decrypted_content = fs::read_to_string(test_file).expect("Failed to read decrypted file");
    assert_eq!(original_content, decrypted_content, "Decrypted content doesn't match original content!");

    cleanup();
}