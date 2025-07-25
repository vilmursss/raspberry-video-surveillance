use std::env;
use std::fs::File;
use std::io::Read;
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

    for request in server.incoming_requests() {
        match request.url() {
            "/" => serve_main_entry_point(request, format!("{}/index.html", HTML_DIR).as_str()),
            _ => not_found(request),
        }
    }
}

fn serve_main_entry_point(request: Request, path: &str) {
    if let Ok(mut file) = File::open(path) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let response = Response::from_string(content)
            .with_header("Content-Type: text/html".parse::<Header>().unwrap());
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