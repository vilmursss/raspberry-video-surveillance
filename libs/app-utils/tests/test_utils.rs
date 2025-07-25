use std::fs;
use tempfile::TempDir;
use app_utils::{copy_file, write_str_to_file};
use app_utils::network::get_interface_ip;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let src_file = temp_dir.path().join("source.txt");
        let dst_file = temp_dir.path().join("destination.txt");

        // Create source file
        fs::write(&src_file, "test content").unwrap();

        // Test copy
        let result = copy_file(
            src_file.to_str().unwrap(),
            dst_file.to_str().unwrap()
        );

        assert!(result);
        assert!(dst_file.exists());

        // Verify content
        let content = fs::read_to_string(&dst_file).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_copy_file_source_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let src_file = temp_dir.path().join("nonexistent.txt");
        let dst_file = temp_dir.path().join("destination.txt");

        let result = copy_file(
            src_file.to_str().unwrap(),
            dst_file.to_str().unwrap()
        );

        assert!(!result);
        assert!(!dst_file.exists());
    }

    #[test]
    fn test_copy_file_overwrite_existing() {
        let temp_dir = TempDir::new().unwrap();
        let src_file = temp_dir.path().join("source.txt");
        let dst_file = temp_dir.path().join("destination.txt");

        // Create both files
        fs::write(&src_file, "new content").unwrap();
        fs::write(&dst_file, "old content").unwrap();

        // Test copy (overwrite)
        let result = copy_file(
            src_file.to_str().unwrap(),
            dst_file.to_str().unwrap()
        );

        assert!(result);

        // Verify content was overwritten
        let content = fs::read_to_string(&dst_file).unwrap();
        assert_eq!(content, "new content");
    }


    #[test]
    fn test_write_str_to_file_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let result = write_str_to_file(file_path.to_str().unwrap(), "Test content");

        assert!(result);
        assert!(file_path.exists());

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content.trim(), "Test content");
    }

    #[test]
    fn test_write_str_to_file_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create existing file
        fs::write(&file_path, "old content").unwrap();

        let result = write_str_to_file(file_path.to_str().unwrap(), "new content");

        assert!(result);

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content.trim(), "new content");
    }

    #[test]
    fn test_write_str_to_file_dir_does_not_exist() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent_dir/test.txt");

        let result = write_str_to_file(file_path.to_str().unwrap(), "Test content");

        assert!(!result);
        assert!(!file_path.exists());
    }

    #[test]
    fn test_get_interface_ip_empty_name() {
        let result = get_interface_ip("");
        assert!(result.is_none());
    }
}