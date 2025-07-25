use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use app_utils::write_str_to_file;

const IP_CMD: &str = "/sbin/ip";
const WPA_SUPPLICANT_CMD: &str = "/usr/sbin/wpa_supplicant";
const UDHCPC_CMD: &str = "/sbin/udhcpc";
const PING_CMD: &str = "/bin/ping";

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
    let file_path = path.join("wifi_connect_result.txt");
    write_str_to_file(file_path.to_str().unwrap_or(""), result)
}

pub fn verify_usb_content(path: &str) -> bool {
    let conf_path = Path::new(path).join("wpa_supplicant.conf");
    return conf_path.exists() && conf_path.is_file()
}

pub fn setup_wifi_connection() -> Result<(), String> {
    // Step 1: Bring down eth0
    println!("Bringing down eth0...");
    let output = Command::new(IP_CMD)
        .args(&["link", "set", "eth0", "down"])
        .output()
        .map_err(|e| format!("Failed to execute '{}': {}", IP_CMD, e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to bring down eth0 (exit code: {}): {}", 
                          output.status.code().unwrap_or(-1), stderr));
    }

    // Step 2: Bring down wlan0
    println!("Bringing down wlan0...");
    let output = Command::new(IP_CMD)
        .args(&["link", "set", "wlan0", "down"])
        .output()
        .map_err(|e| format!("Failed to execute '{}': {}", IP_CMD, e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to bring down wlan0 (exit code: {}): {}", 
                          output.status.code().unwrap_or(-1), stderr));
    }

    // Step 3: Bring up wlan0
    println!("Bringing up wlan0...");
    let output = Command::new(IP_CMD)
        .args(&["link", "set", "wlan0", "up"])
        .output()
        .map_err(|e| format!("Failed to execute '{}': {}", IP_CMD, e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to bring up wlan0 (exit code: {}): {}", 
                          output.status.code().unwrap_or(-1), stderr));
    }

    // Step 4: Start wpa_supplicant
    println!("Starting wpa_supplicant...");
    let output = Command::new(WPA_SUPPLICANT_CMD)
        .args(&["-B", "-i", "wlan0", "-c", "/etc/wpa_supplicant.conf"])
        .output()
        .map_err(|e| format!("Failed to execute '{}': {}", WPA_SUPPLICANT_CMD, e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to start wpa_supplicant (exit code: {}): {}", 
                          output.status.code().unwrap_or(-1), stderr));
    }

    // Wait a bit for wpa_supplicant to establish connection
    println!("Waiting for WiFi connection to establish...");
    thread::sleep(Duration::from_secs(5));

    // Step 5: Get IP address via DHCP
    println!("Requesting IP address via DHCP...");
    let output = Command::new(UDHCPC_CMD)
        .args(&["-i", "wlan0"])
        .output()
        .map_err(|e| format!("Failed to execute '{}': {}", UDHCPC_CMD, e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to get IP address via DHCP (exit code: {}): {}", 
                          output.status.code().unwrap_or(-1), stderr));
    }

    Ok(())
}

pub fn test_connectivity() -> Result<(), String> {
    let test_targets = ["8.8.8.8", "1.1.1.1", "google.com"];
    
    for target in &test_targets {
        println!("Testing connectivity to {}...", target);
        let output = Command::new(PING_CMD)
            .args(&["-c", "3", "-W", "5", target])
            .output()
            .map_err(|e| format!("Failed to execute '{}': {}", PING_CMD, e))?;
        
        if output.status.success() {
            println!("Successfully pinged {}", target);
            return Ok(());
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Failed to ping {} (exit code: {}): {}", 
                    target, output.status.code().unwrap_or(-1), stderr);
        }
    }
    
    Err("Failed to ping any test targets. Internet connectivity may not be available.".to_string())
}