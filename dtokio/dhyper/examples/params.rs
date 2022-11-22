use std::{collections::HashMap, net::SocketAddr};

use hyper::service::service_fn;
use hyper::{server::conn::Http, Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;
use url::form_urlencoded;

extern crate tokio;

static INDEX: &[u8] = b"<html><body><form action=\"post\" method=\"post\">Name: <input type=\"text\" name=\"name\"><br>Number: <input type=\"text\" name=\"number\"><br><input type=\"submit\"></body></html>";
static MISSING: &[u8] = b"Missing field";
static NOTNUMERIC: &[u8] = b"Number field is not numeric";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    let tcp_listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = tcp_listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(param_example))
                .await
            {
                eprintln!("Failed to server connection: {:?}", err);
            }
        });
    }
}

async fn param_example(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(INDEX.into())
            .unwrap()),
        (&Method::POST, "/post") => {
            let body = hyper::body::to_bytes(req).await?;

            let params = form_urlencoded::parse(body.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
            let name = if let Some(n) = params.get("name") {
                n
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };
            let number = if let Some(n) = params.get("number") {
                if let Ok(v) = n.parse::<f64>() {
                    v
                } else {
                    return Ok(Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(NOTNUMERIC.into())
                        .unwrap());
                }
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };
            Ok(Response::new(
                format!("Hello {}, your number is {}", name, number).into(),
            ))
        }
        (&Method::GET, "/get") => {
            let query = if let Some(q) = req.uri().query() {
                q
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };
            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();
            let page = if let Some(p) = params.get("page") {
                p
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };
            Ok(Response::new(format!("You requested {}", page).into()))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}
