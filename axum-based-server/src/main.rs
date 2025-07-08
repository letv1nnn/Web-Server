#![allow(unused)]

use std::net::{SocketAddr, Ipv4Addr};

use tower_http::services::ServeDir;

use axum::Router;
use axum::response::Html;
use axum::routing::get;

use tokio::net::TcpListener;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const SUCCEED_STATUS: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

#[tokio::main]
async fn main() {
    let socket = SocketAddr::from((Ipv4Addr::new(0, 0, 0, 0), 7878));

    let all_routes = Router::new()
        .route("/", get(welcome_route))
        .route("/program", get(program_route))
        .nest_service("/static", ServeDir::new("../web-apps/"))
        .fallback(not_found);

    let result = TcpListener::bind(socket).await;
    if let Ok(listener) = result {
        axum::serve(listener, all_routes.into_make_service()).await.unwrap();
    } else if let Err(e) = result {
        eprintln!("Error occured while binding TCP listener: {}", e);
    }
}

async fn welcome_route() -> Html<String> {
    println!("Welcome page route was proceessed.");
    send_respond("../web-apps/welcome.html").await
}

async fn program_route()-> Html<String> {
    println!("Program editting page route was proceessed.");
    send_respond("../web-apps/upload_program.html").await
}

async fn not_found()-> Html<String> {
    println!("Not found page route was proceessed. Status: {}", NOT_FOUND_STATUS);
    send_respond("../web-apps/404.html").await
}

async fn send_respond(file: &str)  -> Html<String> {
    let mut respond = String::new();
    let result = File::open(file).await;
    if let Ok(mut file) = result {
        file.read_to_string(&mut respond).await.unwrap();
    } else if let Err(e) = result {
        eprintln!("Failed to open the file! Error: {}. Status: {}", e, NOT_FOUND_STATUS);
    }
    println!("{}", SUCCEED_STATUS);
    Html(respond)
}
