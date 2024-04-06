use axum::{
    routing::get,
    Router,
};

extern crate tokio;

#[tokio::main]
async fn main() {
    let app = Router::new();
    let app = app.route(
        "/",
        get(|| async {
            println!("input");
            "Hello axum!"
        }),
    );

    axum::Server::bind(
        &"0.0.0.0:3000"
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}
