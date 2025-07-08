// extern crate server;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use tokio::net::{TcpListener, TcpStream};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::exit;
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // NOTE: configure your firewall to except LAN devices.
    let socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    let listener = match TcpListener::bind(socket).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to bind the socket. Error: {}", e);
            exit(1);
        },
    };

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok((stream, addr)) => (stream, addr),
            Err(_) => {
                println!("Connection failed. Trying again...");
                continue;
            },
        };
        // println!("Connection established: <stream: {:?}>, <addr: {:?}>\n\n", stream, addr);
        println!("New connection from: {}", addr);
        tokio::task::spawn(async move {
            handle_client(stream).await;
        });
    }
}

async fn handle_client(mut stream: TcpStream) {
    let mut buffer: Vec<u8> = vec![0; 1024];
    // n is the number of bytes that were read from the TCP stream into your buffer.
    let n = match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => {
            println!("Client closed the connection.");
            return;
        },
        Ok(n) => n,
        Err(_) => {
            println!("Failed to read the stream!");
            return;
        },
    };
    // You can observe what the server got.
    // println!("Request: {:#?}", String::from_utf8_lossy(&buffer[..n]));

    let request = String::from_utf8_lossy(&buffer[..n]);
    let mut lines = request.lines();
    let request_line = lines.next().unwrap_or("");

    // Handling requests by sending appropriate respond.
    // Can use Axum here to build several roots.
    if buffer.starts_with(b"GET ") {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let path = parts[1];
            let file_path = match path {
                "/" => "../web-apps/welcome.html".to_string(),
                "/sleep" => {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    "../web-apps/welcome.html".to_string()
                },
                _ => {
                    format!("../web-apps{}", path)
                },
            };

            let status = if Path::new(&file_path).exists() {
                "HTTP/1.1 200 OK\r\n\r\n"
            } else {
                "HTTP/1.1 404 NOT FOUND\r\n\r\n"
            };

            let final_path = if Path::new(&file_path).exists() {
                file_path
            } else {
                "../web-apps/404.html".to_string()
            };

            send_respond(stream, status, &final_path).await;
            return;
        }
    }

    send_respond(stream, "HTTP/1.1 404 NOT FOUND\r\n\r\n", "../web-apps/404.html").await;
    
}

// sending appropriate respond
async fn send_respond(mut stream: TcpStream, status: &str, file_name: &str) {
    let mut file = match File::open(file_name).await {
        Ok(res) => res,
        Err(_) => {
            println!("Failed to open the file: <file: {}>", file_name);
            return;
        },
    };

    let mut content = String::new();
    file.read_to_string(&mut content).await.unwrap();
    
    let respond = format!("{}{}", status, content);

    match stream.write_all(respond.as_bytes()).await {
        Ok(_) => {
            stream.flush().await.unwrap();
        },
        Err(_) => {
            println!("Failed to send a respond!");
        },
    }
}
