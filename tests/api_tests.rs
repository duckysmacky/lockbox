mod common;

use std::path::PathBuf;
use common::*;
use lockbox::options::{DeleteKeyOptions, EncryptionOptions, KeyOptions, NewKeyOptions};

#[test]
fn test_new_key() {
    setup();

    let options = NewKeyOptions {
        key_options: KeyOptions {}
    };

    /* Success */
    assert_eq!(lockbox::new_key(PASSWORD, &options), Ok(()));

    /* Invalid password */
    let password_error = lockbox::Error::AuthenticationFailed("Invalid password entered".to_string());
    assert_eq!(lockbox::new_key("1346136134", &options), Err(password_error));

    cleanup();
}

#[test]
fn test_encrypt() {
    setup();

    let mut test_file = PathBuf::from(TEST_DIR);
    test_file.push("word.txt");

    let mut options = EncryptionOptions {
        keep_name: false,
        output_paths: None
    };

    /* Success */
    assert_eq!(lockbox::encrypt(test_file.as_path(), PASSWORD, &mut options), Ok(()));

    /* Invalid password */
    let password_error = lockbox::Error::AuthenticationFailed("Invalid password entered".to_string());
    assert_eq!(lockbox::encrypt(test_file.as_path(), "wrong_password", &mut options), Err(password_error));

    /* Already encrypted */
    let mut encrypted_file = PathBuf::from(TEST_DIR);
    encrypted_file.push("encrypted.box");
    let invalid_file_error = lockbox::Error::InvalidFile(format!("\"{}\" is already encrypted", encrypted_file.display()));
    assert_eq!(lockbox::encrypt(encrypted_file.as_path(), PASSWORD, &mut options), Err(invalid_file_error));

    cleanup();
}

#[test]
fn test_delete_key() {
    setup();

    let delete_options = DeleteKeyOptions {
        key_options: KeyOptions {}
    };

    let key_options = NewKeyOptions {
        key_options: KeyOptions {}
    };

    /* Success */
    assert_eq!(lockbox::new_key(PASSWORD, &key_options), Ok(()));
    assert_eq!(lockbox::delete_key(PASSWORD, &delete_options), Ok(()));

    /* Delete non-existent */
    assert_eq!(lockbox::delete_key("16126161", &delete_options), Ok(()));

    /* Invalid password */
    assert_eq!(lockbox::new_key(PASSWORD, &key_options), Ok(()));
    let password_error = lockbox::Error::AuthenticationFailed("Invalid password entered".to_string());
    assert_eq!(lockbox::delete_key("261613246", &delete_options), Err(password_error));

    cleanup();
}