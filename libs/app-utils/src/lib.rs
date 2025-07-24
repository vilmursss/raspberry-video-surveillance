//! # App Utils
//!
//! Common utility functions for Rust applications.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Copies a file from source to destination path
///
/// # Arguments
///
/// * `src` - A string slice that holds the path to the source file
/// * `dst` - A string slice that holds the path to the destination file
///
/// # Returns
///
/// Returns `true` if the file was copied successfully, `false` otherwise.
/// Error messages are printed to stderr on failure.
pub fn copy_file(src: &str, dst: &str) -> bool {
    let dst_file_path = Path::new(dst);
    use std::fs;

    let src_file_path = Path::new(src);

    match fs::copy(&src_file_path, &dst_file_path) {
        Ok(_) => {
            println!(
                "Successfully copied {:?} to {:?}",
                src_file_path, dst_file_path
            );
            return true
        }
        Err(e) => {
            eprintln!(
                "Failed to copy {:?} to {:?}: {}",
                src_file_path, dst_file_path, e
            );
            return false
        }
    }
}

/// Writes a string to a file, creating or overwriting it
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
/// * `content` - A string slice containing the content to write to the file
///
/// # Returns
///
/// Returns `true` if the content was written successfully, `false` otherwise.
/// Success and error messages are printed to stdout/stderr respectively.
pub fn write_str_to_file(file_path: &str, content: &str) -> bool {
    let path = Path::new(file_path);
    match File::create(&path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", content) {
                eprintln!("Failed to write to file: {}", e);
                return false;
            } else {
                println!("Wrote content to {:?}", path);
            }
        }
        Err(e) => {
            eprintln!("Failed to create file: {:?}", e);
            return false;
        }
    }
    return true;
} 