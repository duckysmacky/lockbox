//! Tests to test how the CLI client performs with different flags and inputs

use std::iter;
use std::path::Path;
use rand::Rng;

mod common;

/// Local test environment setup
fn setup() {
    common::cleanup();
    common::setup();
}

/// Local test environment cleanup
fn cleanup() {
    common::cleanup();
}

/// Macro for fast Lockbox executable command generation and execution. Will initiate a new command
/// with provided arguments, execute it and print its output, returning the resulting `Output`
macro_rules! lockbox_cmd {
    ($cmd:expr) => {
        {
            use common::command::{LockboxCommand, print_output};
            let command = LockboxCommand::new($cmd);
            let output = command.execute();
            print_output(&output);
            output
        }
    };
    ($cmd:expr; $($args:expr),+) => {
        {
            use common::command::{LockboxCommand, print_output};
            let mut command = LockboxCommand::new($cmd);
            for arg in [$($args),*] {
                command.arg(arg);
            }
            let output = command.execute();
            print_output(&output);
            output
        }
    };
}

#[test]
fn test_text_encryption() {
    setup();

    let test_dir = Path::new(common::TEST_DIR);
    let test_file = test_dir.join("text.txt");

    let output = lockbox_cmd!("box"; &test_file);
    assert!(output.status.success(), "Text encryption failed");

    let output = lockbox_cmd!("unbox"; &test_file);
    assert!(output.status.success(), "Text decryption failed");

    cleanup();
}

#[test]
fn test_image_encryption() {
    setup();

    let test_dir = Path::new(common::TEST_DIR);
    let test_file = test_dir.join("image.png");

    let output = lockbox_cmd!("box"; &test_file);
    assert!(output.status.success(), "Image encryption failed");

    let output = lockbox_cmd!("unbox"; &test_file);
    assert!(output.status.success(), "Image decryption failed");

    cleanup();
}

#[test]
fn test_directory_encryption() {
    setup();

    let test_dir = Path::new(common::TEST_DIR);

    let output = lockbox_cmd!("box"; &test_dir);
    assert!(output.status.success(), "Directory encryption failed");

    let output = lockbox_cmd!("unbox"; &test_dir);
    assert!(output.status.success(), "Directory decryption failed");

    cleanup();
}

#[test]
fn test_recursive_encryption() {
    setup();

    let test_dir = Path::new(common::TEST_DIR);

    let output = lockbox_cmd!("box"; &test_dir);
    assert!(output.status.success(), "Recursive encryption failed");

    let output = lockbox_cmd!("unbox"; &test_dir);
    assert!(output.status.success(), "Recursive decryption failed");

    cleanup();
}

#[test]
fn test_profile_manipulation() {
    setup();

    let profile_name: &str = "TEST PROFILE NAME";

    let output = lockbox_cmd!("profile new"; profile_name);
    assert!(output.status.success(), "Profile creation failed");

    let output = lockbox_cmd!("profile select"; profile_name);
    assert!(output.status.success(), "Profile selection failed");

    let output = lockbox_cmd!("profile get");
    assert!(output.status.success(), "Profile name retrieval failed");
    assert!(output.stdout.len() > 0, "Invalid output for current profile");

    let output = lockbox_cmd!("profile list");
    assert!(output.status.success(), "Profiles list retrieval failed");
    assert!(output.stdout.len() > 0, "Invalid output for the list of existing profiles");

    let output = lockbox_cmd!("profile delete"; profile_name);
    assert!(output.status.success(), "Profile deletion failed");

    cleanup();
}

#[test]
fn test_key_generation() {
    setup();

    let test_dir = Path::new(common::TEST_DIR);
    let test_file = test_dir.join("text.txt");

    let output = lockbox_cmd!("key new");
    assert!(output.status.success(), "Key generation failed");

    let output = lockbox_cmd!("box"; &test_file);
    assert!(output.status.success(), "Encryption failed with generated key");

    let output = lockbox_cmd!("unbox"; &test_file);
    assert!(output.status.success(), "Decryption failed with generated key");

    cleanup()
}

#[test]
fn test_key_setting() {
    setup();

    let test_dir = Path::new(common::TEST_DIR);
    let test_file = test_dir.join("text.txt");
    
    // Generate a random 64-byte HEX string
    const CHARSET: &[u8] = b"0123456789ABCDEF";
    let mut rng = rand::thread_rng();
    let valid_key = iter::repeat_with(|| CHARSET[rng.gen_range(0..16)] as char)
        .take(64)
        .collect::<String>();
    let invalid_key = "KY&*";

    let output = lockbox_cmd!("key set"; &valid_key);
    assert!(output.status.success(), "Key set failed");

    let output = lockbox_cmd!("key set"; invalid_key);
    assert!(!output.status.success(), "Invalid key was accepted");

    let output = lockbox_cmd!("box"; &test_file);
    assert!(output.status.success(), "Encryption failed with set key");

    let output = lockbox_cmd!("unbox"; &test_file);
    assert!(output.status.success(), "Decryption failed with set key");

    cleanup();
}