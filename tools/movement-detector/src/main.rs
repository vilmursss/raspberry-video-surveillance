use rppal::gpio::Gpio;
use std::time::Duration;
use chrono::Local;
use std::thread;
use app_utils::add_content_to_file;
use app_utils::WriteMode;

const PIR_PIN: u8 = 27;
const LOG_FILE: &str = "/tmp/movement-detected.log";

fn main() {
    let gpio = Gpio::new().unwrap();
    let pir_input = gpio.get(PIR_PIN).unwrap().into_input();

    loop {
        if pir_input.is_high() {
            // Get current timestamp
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_line = format!("Movement detected at {}", timestamp);

            if !add_content_to_file(LOG_FILE, &log_line, WriteMode::Prepend) {
                eprintln!("Failed to write to log file: {}", LOG_FILE);
            }
        }
        thread::sleep(Duration::from_millis(200));
    }
}