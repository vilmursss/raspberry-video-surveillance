use std::env;
use std::path::Path;
use wifi_connect::{verify_args, verify_usb_content, write_result_to_usb, setup_wifi_connection, test_connectivity};
use app_utils::copy_file;

const WPA_SUPPLICANT_CONF_FILE: &str = "wpa_supplicant.conf";
const WPA_SUPPLICANT_CONF_PATH: &str = "/etc/wpa_supplicant.conf";
const USB_WRITE_ERROR_MESSAGE: &str = "Failed to write to USB";

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Verifying wifi-connect tool arguments");
    if !verify_args(&args) {
        std::process::exit(1);
    }
    
    let mount_dir = &args[1];
    println!("Verifying USB content");
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
    
    println!("Copying {} to {}", WPA_SUPPLICANT_CONF_FILE, WPA_SUPPLICANT_CONF_PATH);
    if !copy_file(&wpa_supplicant_conf_to_copy, WPA_SUPPLICANT_CONF_PATH) {
        let err_msg = format!("Failed to copy {} to {}", WPA_SUPPLICANT_CONF_FILE, WPA_SUPPLICANT_CONF_PATH);
        eprintln!("{}", err_msg);
        if !write_result_to_usb(mount_dir, &err_msg) {
            eprintln!("{}", USB_WRITE_ERROR_MESSAGE);
        }
        std::process::exit(1);
    }

    println!("Setting up WiFi connection");
    if let Err(err_msg) = setup_wifi_connection() {
        let err_msg = format!("WiFi setup failed: {}", err_msg);
        eprintln!("{}", err_msg);
        if !write_result_to_usb(mount_dir, &err_msg) {
            eprintln!("{}", USB_WRITE_ERROR_MESSAGE);
        }
        std::process::exit(1);
    }

    println!("Testing internet connectivity");
    if let Err(err_msg) = test_connectivity() {
        let err_msg = format!("Connectivity test failed: {}", err_msg);
        eprintln!("{}", err_msg);
        if !write_result_to_usb(mount_dir, &err_msg) {
            eprintln!("{}", USB_WRITE_ERROR_MESSAGE);
        }
        std::process::exit(1);
    }

    let success_msg = "WiFi successfully connected";
    println!("{}", success_msg);
    if !write_result_to_usb(mount_dir, success_msg) {
        eprintln!("{}", USB_WRITE_ERROR_MESSAGE);
        std::process::exit(1);
    }
}