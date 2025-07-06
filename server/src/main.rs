extern crate server;

use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::fs::File;
use std::thread;
use std::time::Duration;
use server::ThreadPool;

fn main() -> io::Result<()> {
    // works on local host and LAN (only if the firewall is configured!)
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

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let file_path = "../web-apps/welcome.html";
        respond_with_file(stream, file_path, "HTTP/1.1 200 OK\r\n\r\n")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        let file_path = "../web-apps/welcome.html";
        respond_with_file(stream, file_path, "HTTP/1.1 200 OK\r\n\r\n")
    } else {
        let file_path = "../web-apps/404.html";
        respond_with_file(stream, file_path, "HTTP/1.1 404 NOT FOUND\r\n\r\n")
    }
}

fn respond_with_file(mut stream: TcpStream, file_path: &str, status_line: &str) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let response = format!("{}{}", status_line, content);

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
