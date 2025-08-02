use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};
use tiny_http::{Server, Response, Request, StatusCode, Header};
use app_utils::network::get_interface_ip;
use app_utils::write_str_to_file;

const HTML_DIR: &str = "/usr/share/web-server/html";
const HLS_DIR: &str = "/tmp/hls";

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

    // Ensure HLS directory exists
    if !Path::new(HLS_DIR).exists() {
        if let Err(e) = std::fs::create_dir_all(HLS_DIR) {
            let error_msg = format!("Failed to create HLS directory: {}", e);
            eprintln!("{}", error_msg);
            if !write_str_to_file(&result_file_path, &error_msg) {
                eprintln!("Failed to write error message to USB: {}", result_file_path);
            }
            std::process::exit(1);
        }
    }

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
            "/start-stream" => {
                handle_start_stream(request)
            },
            "/stop-stream" => {
                handle_stop_stream(request)
            },
            "/stream.m3u8" => {
                handle_hls_playlist(request)
            },
            "/styles.css" => {
                handle_static_file(request, format!("{}/styles.css", HTML_DIR).as_str(), "text/css")
            },
            "/script.js" => {
                handle_static_file(request, format!("{}/script.js", HTML_DIR).as_str(), "application/javascript")
            },
            _ if path.starts_with("/segment_") && path.ends_with(".ts") => {
                handle_hls_segment(request, path)
            },
            _ if path.starts_with("segment_") && path.ends_with(".ts") => {
                handle_hls_segment(request, path)
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

fn handle_start_stream(request: Request) {
    println!("Start stream request received");

    // Kill any existing processes
    let _ = Command::new("killall").args(&["-9", "gst-launch-1.0"]).status();

    // Clean up old segments
    let _ = std::fs::remove_dir_all(HLS_DIR);
    let _ = std::fs::create_dir_all(HLS_DIR);

    // Create a basic HLS playlist - don't include any segments yet
    let playlist_content = "#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:3\n#EXT-X-MEDIA-SEQUENCE:0\n";
    if !write_str_to_file(&format!("{}/stream.m3u8", HLS_DIR), &playlist_content) {
        eprintln!("Failed to create initial playlist");
        return send_json_response(request, StatusCode(500), r#"{"error": "Failed to create playlist"}"#);
    }

    // Use a channel to signal when the first segment is ready
    let (tx, rx) = mpsc::channel();

    // Start a thread to handle video capture and segmentation
    thread::spawn(move || {
        if let Err(e) = start_gstreamer_capture(tx) {
            eprintln!("GStreamer capture failed: {}", e);
        }
    });

    // Wait for the first segment to be created (with timeout)
    let wait_result = rx.recv_timeout(Duration::from_secs(15));
    match wait_result {
        Ok(_) => {
            send_json_response(request, StatusCode(200), r#"{"status": "Stream started"}"#)
        },
        Err(_) => {
            eprintln!("Timeout waiting for first segment");
            send_json_response(request, StatusCode(500), r#"{"error": "Failed to start stream"}"#)
        }
    }
}

fn start_gstreamer_capture(tx: mpsc::Sender<()>) -> Result<(), String> {
    let mut first_segment_created = false;

    // Create a directory for segments
    if !Path::new(HLS_DIR).exists() {
        if let Err(e) = std::fs::create_dir_all(HLS_DIR) {
            eprintln!("Failed to create HLS directory: {}", e);
            return Err(format!("Failed to create HLS directory: {}", e));
        }
    }

    // Use hlssink as frontend uses HLS.js library
    let playlist_path = format!("{}/stream.m3u8", HLS_DIR);
    let segment_path = format!("{}/segment_%03d.ts", HLS_DIR);

    let args = [
        "-v",
        "v4l2src", "device=/dev/video0", "!",
        "video/x-raw,width=640,height=480,framerate=15/1", "!",
        "videoconvert", "!",
        "x264enc", "speed-preset=ultrafast", "tune=zerolatency", "key-int-max=15", "!",
        "h264parse", "!",
        "mpegtsmux", "!",
        "hlssink",
        &format!("playlist-location={}", playlist_path),
        &format!("location={}", segment_path),
        "target-duration=2",
        "max-files=10",
        "playlist-length=5"
    ];

    println!("Starting GStreamer with command: gst-launch-1.0 {}",
             args.iter().map(|&s| s).collect::<Vec<&str>>().join(" "));

    let mut child = Command::new("gst-launch-1.0")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start GStreamer: {}", e))?;

    // Monitor the process and check for first segment
    let start_time = SystemTime::now();
    let timeout = Duration::from_secs(10);

    loop {
        // Check if process is still running
        match child.try_wait() {
            Ok(Some(status)) => {
                // Process exited
                let mut error_output = String::new();
                if let Some(mut stderr) = child.stderr.take() {
                    let _ = stderr.read_to_string(&mut error_output);
                }

                if !error_output.is_empty() {
                    eprintln!("GStreamer error: {}", error_output);
                }

                return Err(format!("GStreamer exited unexpectedly: {}", status));
            },
            Ok(None) => {
                // Still running, check for segments
                if !first_segment_created && Path::new(&playlist_path).exists() {
                    // Check if playlist has segments
                    if let Ok(content) = std::fs::read_to_string(&playlist_path) {
                        if content.contains("#EXTINF") {
                            first_segment_created = true;
                            if let Err(e) = tx.send(()) {
                                eprintln!("Failed to send first segment signal: {:?}", e);
                            }
                        }
                    }
                }

                // Check for stop signal
                if Path::new("/tmp/stop_streaming").exists() {
                    println!("Stop signal received, stopping GStreamer");
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }

                // Check timeout
                if !first_segment_created && start_time.elapsed().unwrap_or(Duration::ZERO) > timeout {
                    eprintln!("Timeout waiting for first segment");
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err("Timeout waiting for first segment".to_string());
                }

                // Sleep a bit before checking again
                thread::sleep(Duration::from_millis(100));
            },
            Err(e) => {
                eprintln!("Error checking process status: {}", e);
                return Err(format!("Error checking process status: {}", e));
            }
        }
    }

    Ok(())
}

fn handle_stop_stream(request: Request) {
    println!("Stop stream request received");

    // Signal the capture thread to stop
    let _ = std::fs::write("/tmp/stop_streaming", "stop");

    // Kill any running processes
    let _ = Command::new("killall").args(&["-9", "gst-launch-1.0"]).status();

    // Wait a moment for processes to terminate
    thread::sleep(Duration::from_millis(500));

    // Clean up the signal files
    let _ = std::fs::remove_file("/tmp/stop_streaming");

    send_json_response(request, StatusCode(200), r#"{"status": "Stream stopped"}"#)
}

fn handle_hls_playlist(request: Request) {
    let playlist_path = format!("{}/stream.m3u8", HLS_DIR);

    if Path::new(&playlist_path).exists() {
        match std::fs::read_to_string(&playlist_path) {
            Ok(content) => {
                // Check if the playlist has any segments
                if !content.contains("#EXTINF") {
                    println!("Warning: Playlist has no segments yet");
                }

                let response = Response::from_string(content)
                    .with_header(Header::from_bytes("Content-Type", "application/vnd.apple.mpegurl".as_bytes()).unwrap())
                    .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*".as_bytes()).unwrap())
                    .with_header(Header::from_bytes("Cache-Control", "no-cache, no-store, must-revalidate".as_bytes()).unwrap());

                let _ = request.respond(response);
            },
            Err(e) => {
                eprintln!("Failed to read playlist file: {}", e);
                not_found(request);
            }
        }
    } else {
        eprintln!("Playlist file not found");
        not_found(request);
    }
}

fn handle_hls_segment(request: Request, path: &str) {
    // Extract segment filename from path, handling both /segment_000.ts and segment_000.ts formats
    let segment_filename = path.trim_start_matches('/');
    let segment_path = format!("{}/{}", HLS_DIR, segment_filename);

    if Path::new(&segment_path).exists() {
        match std::fs::metadata(&segment_path) {
            Ok(_) => {
            },
            Err(e) => {
                eprintln!("Failed to get segment metadata: {}", e);
            }
        }

        match std::fs::read(&segment_path) {
            Ok(content) => {
                let response = Response::from_data(content)
                    .with_header(Header::from_bytes("Content-Type", "video/mp2t".as_bytes()).unwrap())
                    .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*".as_bytes()).unwrap())
                    .with_header(Header::from_bytes("Cache-Control", "no-cache, no-store, must-revalidate".as_bytes()).unwrap());

                match request.respond(response) {
                    Ok(_) => {},
                    Err(e) => eprintln!("Error sending segment: {:?}", e),
                }
            },
            Err(e) => {
                eprintln!("Failed to read segment file: {}", e);
                not_found(request);
            }
        }
    } else {
        eprintln!("Segment file not found: {}", segment_path);
        not_found(request);
    }
}

fn send_json_response(request: Request, status: StatusCode, json: &str) {
    let response = Response::from_string(json)
        .with_status_code(status)
        .with_header(Header::from_bytes("Content-Type", "application/json".as_bytes()).unwrap())
        .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*".as_bytes()).unwrap());

    let _ = request.respond(response);
}

fn handle_static_file(request: Request, path: &str, content_type: &str) {
    if let Ok(mut file) = File::open(path) {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let response = Response::from_string(content)
            .with_header(format!("Content-Type: {}", content_type).parse::<Header>().unwrap())
            .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*".as_bytes()).unwrap());
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