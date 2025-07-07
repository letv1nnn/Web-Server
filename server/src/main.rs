extern crate server;

use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;
use std::time::Duration;
use server::ThreadPool;

<<<<<<< HEAD
#[tokio::main]
async fn main() -> io::Result<()> {
=======
fn main() -> io::Result<()> {
    // works on local host and LAN (only if the firewall is configured!)
>>>>>>> 08d617a5a8d2d148714930a67c134789f64669b4
    let addr= "0.0.0.0:8080";
    let listener = match TcpListener::bind(addr) {
        Ok(listener) => {
            println!("Server started successfully on your LAN");
            listener
        },
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return Err(e);
        },
    };

    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    let _ = handle_client(stream);
                });
            },
            Err(e) => {
                eprintln!("Connection failed! Error: {}", e);
            },
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let file_path = "../web-apps/welcome.html";
        respond_with_file(stream, file_path, "HTTP/1.1 200 OK\r\n\r\n");
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        let file_path = "../web-apps/welcome.html";
        respond_with_file(stream, file_path, "HTTP/1.1 200 OK\r\n\r\n");
    } else if buffer.windows(b"GET /".len()).position(|w| w == b"GET /").is_some() {
        let request_line = String::from_utf8_lossy(&buffer);
            if let Some(path) = request_line.lines().next().and_then(|line| line.split_whitespace().nth(1)) {
                if path.ends_with(".css") {
                    let file_path = format!("../web-apps{}", path);
                    return respond_with_file(stream, &file_path, "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\n\r\n");
                }
            }

            let file_path = "../web-apps/404.html";
            respond_with_file(stream, file_path, "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n")
    } else {
        let file_path = "../web-apps/404.html";
        respond_with_file(stream, file_path, "HTTP/1.1 404 NOT FOUND\r\n\r\n");
    }

}

fn respond_with_file(mut stream: TcpStream, file_path: &str, status_line: &str) {
    let mut file = File::open(file_path).unwrap();
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let response = format!("{}{}", status_line, content);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

<<<<<<< HEAD
}
=======
    Ok(())
}
>>>>>>> 08d617a5a8d2d148714930a67c134789f64669b4
