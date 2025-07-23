use std::env;
use std::path::Path;
use wifi_connect::{verify_args, verify_usb_content, write_result_to_usb, copy_file};

const WPA_SUPPLICANT_CONF_FILE: &str = "wpa_supplicant.conf";
const WPA_SUPPLICANT_CONF_PATH: &str = "/etc/wpa_supplicant.conf";
const USB_WRITE_ERROR_MESSAGE: &str = "Failed to write to USB";

fn main() {
    let args: Vec<String> = env::args().collect();

    if !verify_args(&args) {
        std::process::exit(1);
    }
    
    let mount_dir = &args[1];

    if !verify_usb_content(mount_dir) {
        let err_msg = format!("{} not found", WPA_SUPPLICANT_CONF_FILE);
        eprintln!("{}", err_msg);
        if !write_result_to_usb(mount_dir, &err_msg) {
            eprintln!("{}", USB_WRITE_ERROR_MESSAGE);
        }
        std::process::exit(1);
    }

    let wpa_supplicant_conf_to_copy = Path::new(mount_dir)
        .join(WPA_SUPPLICANT_CONF_FILE)
        .to_string_lossy()
        .to_string();
    
    if !copy_file(&wpa_supplicant_conf_to_copy, WPA_SUPPLICANT_CONF_PATH) {
        let err_msg = format!("Failed to copy {} to {}", WPA_SUPPLICANT_CONF_FILE, WPA_SUPPLICANT_CONF_PATH);
        eprintln!("{}", err_msg);
        if !write_result_to_usb(mount_dir, &err_msg) {
            eprintln!("{}", USB_WRITE_ERROR_MESSAGE);
        }
        std::process::exit(1);
    }

    if !write_result_to_usb(mount_dir, "WiFi successfully connected") {
        eprintln!("{}", USB_WRITE_ERROR_MESSAGE);
        std::process::exit(1);
    }
}