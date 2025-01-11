use std::cell::Cell;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::rc::Rc;
use std::task::Poll;

use hyper::body::Bytes;
use hyper::body::HttpBody;
use hyper::rt::Executor;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::Response;

use tokio::net::TcpListener;
use tokio::task::LocalSet;

extern crate tokio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    let local_set = LocalSet::new();

    local_set.block_on(&rt, run())
}

struct Body {
    _marker: PhantomData<*const ()>,
    data: Option<Bytes>,
}

impl From<String> for Body {
    fn from(s: String) -> Self {
        Body {
            _marker: PhantomData,
            data: Some(s.into()),
        }
    }
}

impl HttpBody for Body {
    type Data = Bytes;
    type Error = hyper::Error;

    fn poll_data(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<Self::Data, Self::Error>>> {
        Poll::Ready(self.get_mut().data.take().map(Ok))
    }

    fn poll_trailers(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<Option<hyper::HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}

#[derive(Clone, Copy, Debug)]
struct LocalExec;

impl<F> Executor<F> for LocalExec
where
    F: std::future::Future + 'static,
{
    fn execute(&self, fut: F) {
        tokio::task::spawn_local(fut);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();
    let counter = Rc::new(Cell::new(0));

    let tcp_listener = TcpListener::bind(addr).await?;

    loop {
        let cnt = counter.clone();
        let service = service_fn(move |_| {
            let pre = cnt.get();
            cnt.set(pre + 1);
            let value = cnt.get();

            async move {
                Ok::<_, hyper::Error>(Response::new(Body::from(format!("Request #{}", value))))
            }
        });

        let (stream, _) = tcp_listener.accept().await?;

        tokio::task::spawn_local(async move {
            if let Err(err) = Http::new()
                .with_executor(LocalExec)
                .serve_connection(stream, service)
                .await
            {
                eprintln!("Failed to server connection: {:?}", err);
            }
        });
    }
}
