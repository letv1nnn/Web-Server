extern crate server;

use server::ThreadPool;

use std::io::prelude::*;
use std::net::{self, TcpStream};
use std::fs::File;

use std::thread;
use std::time::Duration;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        pool.execute( || {
            match stream {
                Ok(stream) => handle_connection(stream),
                Err(e) => eprint!("Connection failed: {}", e),
            }
        });
    }
    println!("Shutting down.");
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "get_method.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(3));
        ("HTTP/1.1 200 OK", "get_method.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        },
        Err(_) => {
            format!("<h1>Error: Could not read {} </h1>", filename)
        },
    };

    let response = format!(
        "{}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
