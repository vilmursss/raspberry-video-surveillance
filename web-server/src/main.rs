use std::fs::File;
use std::io::Read;
use tiny_http::{Server, Response, Request, StatusCode, Header};
use get_if_addrs::{get_if_addrs, IfAddr};

const HTML_DIR: &str = "/usr/share/web-server/html";

fn main() {
    let wlan_ip = get_if_addrs()
        .unwrap()
        .into_iter()
        .find_map(|iface| {
            if iface.name == "wlan0" {
                if let IfAddr::V4(v4_addr) = iface.addr {
                    Some(v4_addr.ip)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("No wlan0 IPv4 address found!");

    // Bind to all interfaces to allow external connections
    let bind_addr = "0.0.0.0:8000";
    println!("Server accessible at http://{}:8000/", wlan_ip);
    println!("Server binding to all interfaces: {}", bind_addr);

    let server = Server::http(bind_addr).unwrap();

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