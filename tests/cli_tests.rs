use std::path::PathBuf;
use std::process::Output;
use crate::common::TEST_DIR;

mod common;

#[allow(dead_code)]
mod commands {
    use std::path::PathBuf;
    use std::process::{Command, Output};
    use super::common::PASSWORD;

    pub fn run_box(path: &PathBuf, extra_args: &[&str]) -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "box", path.to_str().unwrap()])
            .args(extra_args)
            .output()
            .expect("Failed to execute the \"box\" command")
    }

    pub fn run_unbox(path: &PathBuf, extra_args: &[&str]) -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "unbox", path.to_str().unwrap()])
            .args(extra_args)
            .output()
            .expect("Failed to execute the \"unbox\" command")
    }

    pub fn run_profile_new(name: &str) -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "profile", "new", name])
            .output()
            .expect("Failed to execute the \"profile create\" command")
    }

    pub fn run_profile_delete(name: &str) -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "profile", "delete", name])
            .output()
            .expect("Failed to execute the \"profile delete\" command")
    }

    pub fn run_profile_select(name: &str) -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "profile", "select", name])
            .output()
            .expect("Failed to execute the \"profile select\" command")
    }

    pub fn run_profile_get() -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "profile", "get"])
            .output()
            .expect("Failed to execute the \"profile get\" command")
    }

    pub fn run_profile_list() -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "profile", "list"])
            .output()
            .expect("Failed to execute the \"profile list\" command")
    }

    pub fn run_key_new() -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "key", "new"])
            .output()
            .expect("Failed to execute the \"key new\" command")
    }

    pub fn run_key_get() -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "key", "get"])
            .output()
            .expect("Failed to execute the \"key get\" command")
    }

    pub fn run_key_set(key: &str) -> Output {
        Command::new("lockbox")
            .args(&["-v", "-p", PASSWORD, "key", "set", key])
            .output()
            .expect("Failed to execute the \"key set\" command")
    }
}

fn print_output(output: &Output) {
    println!(
        "STDOUT:\n{}\nSTDERR:\n{}\nSTATUS:\n{}\n",
        std::str::from_utf8(&output.stdout).unwrap(),
        std::str::from_utf8(&output.stderr).unwrap(),
        output.status
    )
}

fn setup() {
    common::setup();

    let output = commands::run_profile_new("test");
    print_output(&output);
    assert!(output.status.success(), "Profile creation failed");

    let output = commands::run_profile_select("test");
    print_output(&output);
    assert!(output.status.success(), "Profile selection failed");
}

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