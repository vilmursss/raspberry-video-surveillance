use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn verify_args(args: &[String]) -> bool {
    if args.len() != 2 {
        eprintln!("Usage: {} <mount_directory>", args[0]);
        return false;
    }

    let mount_dir = &args[1];
    let path = Path::new(mount_dir);

    if !path.exists() || !path.is_dir() {
        eprintln!("Error: '{}' does not exist or is not a directory.", mount_dir);
        return false;
    }

    return true;
}

pub fn write_result_to_usb(mount_dir: &str, result: &str) -> bool {
    let path = Path::new(mount_dir);
    let file_path = path.join("result.txt");
    match File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", result) {
                eprintln!("Failed to write to file: {}", e);
                return false;
            } else {
                println!("Wrote '{}' to {:?}", result, file_path);
            }
        }
        Err(e) => {
            eprintln!("Failed to create file: {:?}", e);
            return false;
        }
    }
    return true;
} 