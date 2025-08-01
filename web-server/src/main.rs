use std::env;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use tiny_http::{Server, Response, Request, StatusCode, Header};
use app_utils::network::get_interface_ip;
use app_utils::write_str_to_file;

const HTML_DIR: &str = "/usr/share/web-server/html";

fn main() {
    // Get USB mount directory from command line arguments
    let args: Vec<String> = env::args().collect();
    let usb_mount_path = if args.len() > 1 {
        &args[1]
    } else {
        std::process::exit(1);
    };

    println!("Using USB mount path for web-server: {}", usb_mount_path);
    let result_file_path = format!("{}/web_server_result.txt", usb_mount_path);

    let wlan_ip = match get_interface_ip("wlan0") {
        Some(ip) => ip,
        None => {
            let error_msg = "Failed to retrieve wlan0 IP address";
            eprintln!("{}", error_msg);

            if !write_str_to_file(&result_file_path, error_msg) {
                eprintln!("Failed to write error message to USB: {}", result_file_path);
            }
            std::process::exit(1);
        }
    };

    // Bind to all interfaces to allow external connections
    let bind_addr = "0.0.0.0:8000";
    println!("Server accessible at http://{}:8000/", wlan_ip);
    println!("Server binding to all interfaces: {}", bind_addr);

    let server = Server::http(bind_addr).unwrap();

    write_str_to_file(&result_file_path,
        &format!("Web server started successfully and running in http://{}:8000/", wlan_ip));

    println!("Server started, waiting for requests...");

    for request in server.incoming_requests() {
        let full_url = request.url().to_string();

        // Extract the path part of the URL (without query parameters)
        let path = match full_url.find('?') {
            Some(pos) => &full_url[..pos],
            None => &full_url
        };

        match path {
            "/" => {
                handle_main_page(request, format!("{}/index.html", HTML_DIR).as_str())
            },
            "/snapshot" => {
                handle_snapshot(request)
            },
            "/styles.css" => {
                handle_static_file(request, format!("{}/styles.css", HTML_DIR).as_str(), "text/css")
            },
            "/script.js" => {
                handle_static_file(request, format!("{}/script.js", HTML_DIR).as_str(), "application/javascript")
            },
            _ => {
                println!("Handling 404 for {}", full_url);
                not_found(request)
            },
        }

    }
}

fn handle_main_page(request: Request, path: &str) {
    handle_static_file(request, path, "text/html")
}

fn handle_snapshot(request: Request) {
    let temp_file = format!("/tmp/snapshot_{}.jpg", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs());

    // Build the command
    let raspistill_args = [
        "-o", &temp_file,     // Output to file
        "-w", "640",          // Width
        "-h", "480",          // Height
        "-t", "1",            // Minimal delay (1ms)
        "-n",                 // No preview
        "-ex", "auto",        // Auto exposure
        "-awb", "auto",       // Auto white balance
        "-q", "75"            // JPEG quality
    ];

    // Capture the image to a file
    let status = Command::new("raspistill")
        .args(&raspistill_args)
        .status();

    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                eprintln!("raspistill command failed with status: {}", exit_status);
                return not_found(request);
            }
        },
        Err(e) => {
            eprintln!("Failed to execute raspistill: {}", e);
            return not_found(request);
        }
    }

    match File::open(&temp_file) {
        Ok(mut file) => {
            let mut jpeg_data = Vec::new();
            match file.read_to_end(&mut jpeg_data) {
                Ok(bytes_read) => {
                    let headers = vec![
                        Header::from_bytes("Content-Type", "image/jpeg".as_bytes()).unwrap(),
                        Header::from_bytes("Content-Length", bytes_read.to_string().as_bytes()).unwrap(),
                    ];

                    let response = Response::new(
                        StatusCode(200),
                        headers,
                        std::io::Cursor::new(jpeg_data),
                        Some(bytes_read),
                        None
                    );

                    match std::fs::remove_file(&temp_file) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Failed to remove temporary file: {}", e),
                    }

                    match request.respond(response) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error sending snapshot: {:?}", e),
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read temporary file: {}", e);
                    not_found(request);
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to open temporary file: {}", e);
            not_found(request);
        }
    }
}

fn handle_static_file(request: Request, path: &str, content_type: &str) {
    if let Ok(mut file) = File::open(path) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let response = Response::from_string(content)
            .with_header(format!("Content-Type: {}", content_type).parse::<Header>().unwrap());
        let _ = request.respond(response);
    } else {
        println!("Failed to open file: {}", path);
        not_found(request);
    }
}

fn not_found(request: Request) {
    let response = Response::new_empty(StatusCode(404));
    let _ = request.respond(response);
}