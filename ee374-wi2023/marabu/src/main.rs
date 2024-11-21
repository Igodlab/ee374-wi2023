#![allow(unused)]

// use self::error::Error;
use axum::{Router, response::Html, routing::get};
use tokio::net::{TcpListener, TcpStream};

// >>> Handshake with other peers >>>
async fn connect_to_peer(
    addr: &str,
) -> Result<TcpStream, Box<dyn Error>> {
    println!("Attempting to connect to peer at {addr}");
    let stream = TcpStream::connect(addr).await?;
    println!("Successfully connected to peer at {addr}");
    Ok(stream)
}
// <<< Handshake with other peers <<<

#[tokio::main]
async fn main() {
    let routes_merged = Router::new()
        .route(
            "/hello", 
            get(|| async { Html("Welcome to the <strong>Marabu</strong> Client Node!") }),
        );

    // >>> Start Server >>>
    let listener = TcpListener::bind("127.0.0.1:18018").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_merged.into_make_service())
        .await
        .unwrap();
    // <<< Start Server <<<
    // Ok(())
}
