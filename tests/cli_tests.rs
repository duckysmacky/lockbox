//! Tests to test how the CLI client performs with different flags and inputs

use std::path::PathBuf;
use std::process::Output;
use crate::common::{commands, TEST_DIR};

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

    let output = commands::run_profile_new("test");
    print_output(&output);
    assert!(output.status.success(), "Profile creation failed");

    let output = commands::run_profile_select("test");
    print_output(&output);
    assert!(output.status.success(), "Profile selection failed");
}

/// Local test environment cleanup
fn cleanup() {
    common::cleanup();

    let output = commands::run_profile_delete("test");
    print_output(&output);
    assert!(output.status.success(), "Profile deletion failed");
}

#[test]
fn test_text_encryption() {
    setup();

    let test_dir = PathBuf::from(TEST_DIR);
    let test_file = test_dir.join("text.txt");

    let output = commands::run_box(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Encryption failed");

    let output = commands::run_unbox(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Decryption failed");

    cleanup();
}

#[test]
fn test_image_encryption() {
    setup();

    let test_dir = PathBuf::from(TEST_DIR);
    let test_file = test_dir.join("image.png");

    let output = commands::run_box(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Encryption failed");

    let output = commands::run_unbox(&test_file, &[]);
    print_output(&output);
    assert!(output.status.success(), "Decryption failed");

    cleanup();
}

#[test]
fn test_directory_encryption() {
    setup();

    let test_dir = PathBuf::from(TEST_DIR);

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

    let test_dir = PathBuf::from(TEST_DIR);

    let output = commands::run_box(&test_dir, &["-R"]);
    print_output(&output);
    assert!(output.status.success(), "Recursive encryption failed");

    let output = commands::run_unbox(&test_dir, &["-R"]);
    print_output(&output);
    assert!(output.status.success(), "Recursive decryption failed");

    cleanup();
}