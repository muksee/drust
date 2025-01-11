use std::{
    fmt::{self, Debug, Display},
    future::Future,
    task::Poll,
    time::Duration,
};

use pin_project_lite::pin_project;
use tokio::time::Sleep;
use tower::{Layer, Service};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn main() {}

#[derive(Debug, Clone)]
struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S> Timeout<S> {
    fn new(inner: S, timeout: Duration) -> Self {
        Self { inner, timeout }
    }
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let response_future = self.inner.call(req);
        let sleep_future = tokio::time::sleep(self.timeout);

        ResponseFuture {
            response_future,
            sleep_future,
        }
    }
}

pin_project! {
    pub struct ResponseFuture<F> {
        #[pin]
        response_future: F,
        #[pin]
        sleep_future: Sleep,
    }
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let response_future = this.response_future;
        let sleep_future = this.sleep_future;

        match response_future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }

        match sleep_future.poll(cx) {
            Poll::Ready(_) => {
                let error = Box::new(TimeoutError(()));
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

#[derive(Debug, Default)]
pub struct TimeoutError(());

impl std::error::Error for TimeoutError {}
impl Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("request timeout")
    }
}

#[derive(Debug, Clone)]
struct TimeoutLayer {
    timeout: Duration,
}

#[allow(unused)]
impl TimeoutLayer {
    fn new(timeout: Duration) -> Self {
        TimeoutLayer { timeout }
    }
}

impl<S> Layer<S> for TimeoutLayer
where
    S: Debug + Clone,
{
    type Service = Timeout<S>;
    fn layer(&self, inner: S) -> Self::Service {
        Timeout::new(inner, self.timeout)
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::error_handling::HandleErrorLayer;
    use axum::http::request;
    use axum::routing::get;
    use axum::{BoxError, Router};
    use hyper::StatusCode;
    use std::future::Future;
    use std::pin::Pin;
    use std::time::Duration;
    use tower::{Service, ServiceBuilder};

    use crate::{TimeoutError, TimeoutLayer};

    async fn handler() -> &'static str {
        tokio::time::sleep(Duration::from_secs(2)).await;
        "Hello, Timeout"
    }

    async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
        if err.is::<TimeoutError>() {
            (StatusCode::REQUEST_TIMEOUT, "request time out".to_owned())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("error: {err}"))
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_timeout() {
        let layer = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .layer(TimeoutLayer::new(Duration::from_secs(1)));

        let mut app = Router::new().route("/", get(handler)).layer(layer);

        let req = request::Builder::new()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let output = app.call(req).await;

        assert_eq!(StatusCode::OK, output.unwrap().status());
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_timeout2() {
        let layer = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .layer(TimeoutLayer::new(Duration::from_secs(4)));

        let mut app = Router::new().route("/", get(handler)).layer(layer);

        let req = request::Builder::new()
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let output = app.call(req).await;

        assert_eq!(StatusCode::REQUEST_TIMEOUT, output.unwrap().status());
    }

    #[derive(Debug, Clone)]
    struct EmptyService;

    impl EmptyService {
        fn new() -> Self {
            EmptyService
        }
    }

    impl Service<DelayPower> for EmptyService {
        type Response = i32;
        type Error = BoxError;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

        fn poll_ready(
            &mut self,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            std::task::Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: DelayPower) -> Self::Future {
            Box::pin(async move {
                tokio::time::sleep(Duration::from_secs(req.1)).await;
                Ok(req.0 * req.0)
            })
        }
    }

    struct DelayPower(i32, u64);

    impl DelayPower {
        fn new(n: i32, delay: u64) -> Self {
            DelayPower(n, delay)
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_timeout3() {
        let service = EmptyService::new();
        let mut layer = ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(4)))
            .service(service);

        for i in 0..10 {
            println!("{}", i);
            let req = DelayPower::new(100, i);
            let result = layer.call(req).await;

            println!("{:?}", result);
        }
    }
}
