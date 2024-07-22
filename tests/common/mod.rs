use std::fs;
use std::path::Path;

pub const ORIGINAL_DIR: &str = "test-files/original";
pub const TEST_DIR: &str = "test-files/test";

pub fn setup() {
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

pub fn cleanup() {
    let test_path = Path::new(TEST_DIR);
    if test_path.exists() {
        fs::remove_dir_all(test_path).unwrap();
    }
}