#![allow(unused)]

// mod error;

// use self::error::Error;
use axum::{Router, response::Html, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_merged = Router::new()
        .route(
            "/hello", 
            get(|| async { Html("Welcome to the Marabu Client Node!") }),
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
