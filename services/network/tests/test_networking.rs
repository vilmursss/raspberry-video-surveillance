use std::fs;
use std::io::Read;
use tempfile::TempDir;

use network::{verify_args, write_result_to_usb};

#[test]
fn test_verify_args_insufficient_arguments() {
    let args = vec!["program_name".to_string()];
    assert_eq!(verify_args(&args), false);
}

#[test]
fn test_verify_args_too_many_arguments() {
    let args = vec![
        "program_name".to_string(),
        "dir1".to_string(),
        "dir2".to_string(),
        "extra".to_string(),
    ];
    assert_eq!(verify_args(&args), false);
}

#[test]
fn test_verify_args_nonexistent_directory() {
    let args = vec![
        "program_name".to_string(),
        "/this/path/does/not/exist".to_string(),
    ];
    assert_eq!(verify_args(&args), false);
}

#[test]
fn test_verify_args_file_instead_of_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    fs::File::create(&file_path).unwrap();

    let args = vec![
        "program_name".to_string(),
        file_path.to_string_lossy().to_string(),
    ];
    assert_eq!(verify_args(&args), false);
}

#[test]
fn test_verify_args_valid_directory() {
    let temp_dir = TempDir::new().unwrap();
    let args = vec![
        "program_name".to_string(),
        temp_dir.path().to_string_lossy().to_string(),
    ];
    assert_eq!(verify_args(&args), true);
}

#[test]
fn test_write_result_to_usb_success() {
    let temp_dir = TempDir::new().unwrap();
    let mount_dir = temp_dir.path().to_string_lossy();
    let test_message = "Test message";

    assert_eq!(write_result_to_usb(&mount_dir, test_message), true);

    let result_file = temp_dir.path().join("result.txt");
    assert!(result_file.exists());

    let mut file_content = String::new();
    fs::File::open(&result_file)
        .unwrap()
        .read_to_string(&mut file_content)
        .unwrap();

    assert_eq!(file_content.trim(), test_message);
}

#[test]
fn test_write_result_to_usb_overwrite_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let mount_dir = temp_dir.path().to_string_lossy();
    
    // Create initial file with some content
    let result_file = temp_dir.path().join("result.txt");
    fs::write(&result_file, "Initial content").unwrap();

    let new_message = "Overwritten content";
    assert_eq!(write_result_to_usb(&mount_dir, new_message), true);

    let mut file_content = String::new();
    fs::File::open(&result_file)
        .unwrap()
        .read_to_string(&mut file_content)
        .unwrap();

    assert_eq!(file_content.trim(), new_message);
}

#[test]
fn test_write_result_to_usb_nonexistent_directory() {
    let nonexistent_path = "/this/path/does/not/exist";
    let test_message = "This should fail";

    assert_eq!(write_result_to_usb(nonexistent_path, test_message), false);
}