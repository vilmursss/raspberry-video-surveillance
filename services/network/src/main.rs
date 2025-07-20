use std::env;
use network::{verify_args, write_result_to_usb};

fn main() {
    let args: Vec<String> = env::args().collect();

    if !verify_args(&args) {
        std::process::exit(1);
    }
    
    let mount_dir = &args[1];

    if !write_result_to_usb(mount_dir, "USB Successfully Mounted") {
        eprintln!("Failed to write to USB");
        std::process::exit(1);
    }
}