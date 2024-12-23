#![allow(dead_code)]
//! Contains command templates for running CLI tests. The returned Output should be used to
//! determine if the test was successful or not

use std::process::{Command, Output};
use std::path::PathBuf;
use super::PASSWORD;

fn base_command() -> Command {
    Command::new("target/debug/lockbox.exe")
}

pub fn run_box(path: &PathBuf, extra_args: &[&str]) -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "box", path.to_str().unwrap()])
        .args(extra_args)
        .output()
        .expect("Failed to execute the \"box\" command")
}

pub fn run_unbox(path: &PathBuf, extra_args: &[&str]) -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "unbox", path.to_str().unwrap()])
        .args(extra_args)
        .output()
        .expect("Failed to execute the \"unbox\" command")
}

pub fn run_profile_new(name: &str) -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "profile", "new", name])
        .output()
        .expect("Failed to execute the \"profile create\" command")
}

pub fn run_profile_delete(name: &str) -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "profile", "delete", name])
        .output()
        .expect("Failed to execute the \"profile delete\" command")
}

pub fn run_profile_select(name: &str) -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "profile", "select", name])
        .output()
        .expect("Failed to execute the \"profile select\" command")
}

pub fn run_profile_get() -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "profile", "get"])
        .output()
        .expect("Failed to execute the \"profile get\" command")
}

pub fn run_profile_list() -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "profile", "list"])
        .output()
        .expect("Failed to execute the \"profile list\" command")
}

pub fn run_key_new() -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "key", "new"])
        .output()
        .expect("Failed to execute the \"key new\" command")
}

pub fn run_key_get() -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "key", "get"])
        .output()
        .expect("Failed to execute the \"key get\" command")
}

pub fn run_key_set(key: &str) -> Output {
    base_command()
        .args(&["-v", "-p", PASSWORD, "key", "set", key])
        .output()
        .expect("Failed to execute the \"key set\" command")
}