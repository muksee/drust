use std::{convert::Infallible, net::SocketAddr};

use futures::TryStreamExt;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};

extern crate tokio;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // CTRL+C Graceful stop
    // 必须传入Future对象,而异步函数执行返回Future对象
    let server = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler!")
}

async fn hello_world(
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    match (
        req.method(),
        req.uri()
            .path(),
    ) {
        (&Method::GET, "/get") => {
            *response.body_mut() = Body::from("I am from GET /get");
        }
        (&Method::POST, "/post") => {
            *response.body_mut() = req.into_body();
        }
        (&Method::POST, "/post/upper") => {
            let mapping = req
                .into_body()
                .map_ok(|chunk| {
                    chunk
                        .iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });

            *response.body_mut() = Body::wrap_stream(mapping);
        }
        (&Method::POST, "/post/reverse") => {
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let reversed = full_body
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();
            *response.body_mut() = reversed.into();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

