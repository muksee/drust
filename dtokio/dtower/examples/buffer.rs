use std::convert::Infallible;
use std::time::Duration;

use tower::service_fn;
use tower::{buffer::Buffer, Service, ServiceExt};

extern crate tokio;

async fn mass_produce<S: Service<usize>>(svc: S)
where
    S: 'static + Send,
    S::Error: Send + Sync + std::error::Error,
    S::Future: Send,
{
    let svc = Buffer::new(svc, 1);

    let mut handles = vec![];
    for _ in 0..4 {
        let mut svc = svc.clone();
        let handle = tokio::spawn(async move {
            for i in 0usize..5 {
                let _rsp = svc.ready().await.expect("service crashed").call(i).await;
            }
        });
        handles.push(handle);
    }

    for h in handles {
        _ = h.await;
    }
}

async fn handle(request: usize) -> Result<usize, Infallible> {
    println!("service processing begin {}", request);
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("service processing end {}", request);
    Ok(request * request)
}

#[tokio::main]
async fn main() {
    let svc = service_fn(handle);
    mass_produce(svc).await;
}
