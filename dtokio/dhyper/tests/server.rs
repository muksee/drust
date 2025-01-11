use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use tower::make::Shared;

async fn handle1(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, server!")))
}
#[tokio::test]
async fn server1() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let make_svc =
        make_service_fn(|_conn: &AddrStream| async { Ok::<_, Infallible>(service_fn(handle1)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}

#[derive(Clone, Debug)]
struct AppContext {
    session: String,
}

async fn handle2(
    context: AppContext,
    addr: SocketAddr,
    _: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(format!(
        "Hello, server, from {}, context {}",
        addr, context.session
    ))))
}

#[tokio::test]
async fn server2() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let context = AppContext {
        session: String::from("null"),
    };

    let make_svc = make_service_fn(|conn: &AddrStream| {
        let context = context.clone();
        let addr = conn.remote_addr();
        let service = service_fn(move |req| handle2(context.clone(), addr, req));

        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}

async fn handle3(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, shared service server!")))
}

#[tokio::test]
async fn server3() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let make_svc = Shared::new(service_fn(handle3));

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }
}
