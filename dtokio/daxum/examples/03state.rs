use axum::routing::get;
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;

extern crate tokio;

struct State {
    name: String,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(State {
        name: String::from("Lily"),
    });
    let app = Router::new()
        .route("/state1", get(state1))
        .route("/state2", get(state2))
        .layer(Extension(shared_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn state1(Extension(state): Extension<Arc<State>>) -> String {
    state.name.clone()
}

async fn state2(Extension(state): Extension<Arc<State>>) -> String {
    state.name.clone()
}
