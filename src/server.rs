use std::net::SocketAddr;

use axum::{routing::{get, post}, Router};

use crate::router::controller::controller;

pub async fn server() {
    let app = Router::new()
                .route("/", get(root))
                .route("/keys", post(controller));


    let addr = SocketAddr::from(([127,0,0,1], 3333));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}