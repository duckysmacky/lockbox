//! Tests to test how the CLI client performs with different flags and inputs

use std::iter;
use std::path::PathBuf;
use std::process::Output;
use rand::Rng;
use crate::common::commands;

mod common;

fn print_output(output: &Output) {
    println!(
        "STDOUT:\n{}\nSTDERR:\n{}\nSTATUS:\n{}\n",
        std::str::from_utf8(&output.stdout).unwrap(),
        std::str::from_utf8(&output.stderr).unwrap(),
        output.status
    )
}

/// Local test environment setup
fn setup() {
    common::setup();
}

/// Local test environment cleanup
fn cleanup() {
    common::cleanup();
}

#[test]
fn test_text_encryption() {
    setup();

    let test_dir = PathBuf::from(common::TEST_DIR);
    let test_file = test_dir.join("text.txt");

    let output = commands::run_box(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Text encryption failed");

    let output = commands::run_unbox(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Text decryption failed");

    cleanup();
}

#[test]
fn test_image_encryption() {
    setup();

    let test_dir = PathBuf::from(common::TEST_DIR);
    let test_file = test_dir.join("image.png");

    let output = commands::run_box(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Image encryption failed");

    let output = commands::run_unbox(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Image decryption failed");

    cleanup();
}

#[test]
fn test_directory_encryption() {
    setup();

    let test_dir = PathBuf::from(common::TEST_DIR);

    let output = commands::run_box(&test_dir, &[]);
    print_output(&output);
    assert!(output.status.success(), "Directory encryption failed");

    let output = commands::run_unbox(&test_dir, &[]);
    print_output(&output);
    assert!(output.status.success(), "Directory decryption failed");

    cleanup();
}

#[test]
fn test_recursive_encryption() {
    setup();

    let test_dir = PathBuf::from(common::TEST_DIR);

    let output = commands::run_box(&test_dir, &["-R"]);
    print_output(&output);
    assert!(output.status.success(), "Recursive encryption failed");

    let output = commands::run_unbox(&test_dir, &["-R"]);
    print_output(&output);
    assert!(output.status.success(), "Recursive decryption failed");

    cleanup();
}

#[test]
fn test_profile_manipulation() {
    setup();

    let profile_name: &str = "TEST-PROFILE-NAME";

    let output = commands::run_profile_new(profile_name);
    print_output(&output);
    assert!(output.status.success(), "Profile creation failed");

    let output = commands::run_profile_select(profile_name);
    print_output(&output);
    assert!(output.status.success(), "Profile selection failed");

    let output = commands::run_profile_get();
    print_output(&output);
    assert!(output.status.success(), "Profile name retrieval failed");
    assert!(output.stdout.len() > 0, "Invalid output for current profile");

    let output = commands::run_profile_list();
    print_output(&output);
    assert!(output.status.success(), "Profiles list retrieval failed");
    assert!(output.stdout.len() > 0, "Invalid output for the list of existing profiles");

    let output = commands::run_profile_delete(profile_name);
    print_output(&output);
    assert!(output.status.success(), "Profile deletion failed");

    cleanup();
}

#[test]
fn test_key_generation() {
    setup();

    let test_dir = PathBuf::from(common::TEST_DIR);
    let test_file = test_dir.join("text.txt");

    let output = commands::run_key_new();
    print_output(&output);
    assert!(output.status.success(), "Key generation failed");

    let output = commands::run_box(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Encryption failed with generated key");

    let output = commands::run_unbox(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Decryption failed with generated key");

    cleanup()
}

#[test]
fn test_key_setting() {
    setup();

    let test_dir = PathBuf::from(common::TEST_DIR);
    let test_file = test_dir.join("text.txt");
    
    // Generate a random 64-byte HEX string
    const CHARSET: &[u8] = b"0123456789ABCDEF";
    let mut rng = rand::thread_rng();
    let valid_key = iter::repeat_with(|| CHARSET[rng.gen_range(0..16)] as char)
        .take(64)
        .collect::<String>();
    let invalid_key = "KY&*";

    let output = commands::run_key_set(&valid_key);
    print_output(&output);
    assert!(output.status.success(), "Key set failed");

    let output = commands::run_key_set(invalid_key);
    print_output(&output);
    assert!(!output.status.success(), "Invalid key was accepted");

    let output = commands::run_box(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Encryption failed with set key");

    let output = commands::run_unbox(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Decryption failed with set key");

    cleanup();
}