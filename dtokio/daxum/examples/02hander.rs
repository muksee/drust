extern crate tokio;

use std::net::SocketAddr;

use axum::{
    routing::{
        get,
        post,
    },
    Router,
};
use hyper::{
    body::Bytes,
    StatusCode,
};

#[tokio::main]
async fn main() {
    let app = Router::new();
    let app = app
        .route("/unit", get(unit_handler))
        .route("/string", get(string_handler))
        .route("/echo", post(echo));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// empty handler
async fn unit_handler() {}

// string handler
async fn string_handler() -> String {
    String::from("Hello Axum\n")
}

// echo handler
async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
