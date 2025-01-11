//#![deny(warnings)]

extern crate tokio;

use std::net::SocketAddr;
use std::vec;

use hyper::body::Buf;
use hyper::header;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;

use tokio::net::TcpListener;
use tokio::net::TcpStream;


type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
static NOTFOUND: &[u8] = b"Not Found";
static POST_DATA: &str = r#"{"original": "data"}"#;
static URL: &str = "http://127.0.0.1:8000/json_api";

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let service = service_fn(response_examples);
            if let Err(err) = Http::new().serve_connection(stream, service).await {
                eprintln!("Failed to server connection: {:?}", err);
            }
        });
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    println!("Request:{:?}", req);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(INDEX.into())),
        (&Method::GET, "/json_api") => api_get_response(req).await,
        (&Method::POST, "/json_api") => api_post_response(req).await,
        (&Method::GET, "/test.html") => client_request_response(req).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(NOTFOUND.into())
            .unwrap()),
    }
}

async fn api_get_response(_req: Request<Body>) -> Result<Response<Body>> {
    let data = vec!["Hello", "Hyper"];
    let res = match serde_json::to_string(&data) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };

    Ok(res)
}

async fn api_post_response(req: Request<Body>) -> Result<Response<Body>> {
    let body = hyper::body::aggregate(req).await?;
    let mut data: serde_json::Value = serde_json::from_reader(body.reader())?;
    data["test"] = serde_json::Value::from("test value");

    let json = serde_json::to_string(&data)?;
    let res = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))?;

    Ok(res)
}
async fn client_request_response(_req: Request<Body>) -> Result<Response<Body>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri(URL)
        .header(header::CONTENT_TYPE, "application/json")
        .body(POST_DATA.into())
        .unwrap();

    let host = req.uri().host().expect("uri has no host");
    let port = req.uri().port_u16().expect("uri has no port");
    let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;

    let (mut sender, conn) = hyper::client::conn::handshake(stream).await?;

    tokio::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection error:{:?}", err);
        }
    });

    let web_res = sender.send_request(req).await?;
    let res_body = web_res.into_body();

    Ok(Response::new(res_body))
}
