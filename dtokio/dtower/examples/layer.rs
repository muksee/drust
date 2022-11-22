use core::fmt;

use tower::{Layer, Service};

#[tokio::main]
async fn main() {}

pub struct LogService<S> {
    inner: S,
    target: &'static str,
}

impl<S, R> Service<R> for LogService<S>
where
    S: Service<R>,
    R: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: R) -> Self::Future {
        println!("1-->request={:?}, target={:?}", req, self.target);
        self.inner.call(req)
    }
}

pub struct LogLayer {
    target: &'static str,
}

impl LogLayer {
    fn new(target: &'static str) -> Self {
        LogLayer { target }
    }
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LogService {
            inner,
            target: self.target,
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use tower::{layer::layer_fn, service_fn, Layer, Service, ServiceBuilder};

    use crate::{LogLayer, LogService};
    use std::fmt;

    #[tokio::test]
    async fn test_layer_with_builder() {
        let c = |req| async move {
            println!("2-->process request {:?}", req);
            Ok::<_, Infallible>(req)
        };

        let s = service_fn(c);

        let mut outer = ServiceBuilder::new()
            .layer(LogLayer::new("test_layer_with_builder"))
            .service(s);

        let ret = outer.call("Hello").await.unwrap();
        println!("Result:{ret}");
    }

    #[tokio::test]
    async fn test_layer_with_wrapper() {
        let s = service_fn(|req| async move {
            println!("2-->process request {:?}", req);
            Ok::<_, Infallible>(req)
        });

        let mut outer = LogLayer::new("test_layer_with_wrapper").layer(s);

        let ret = outer.call("Hello").await.unwrap();
        println!("Result:{ret}");
    }

    #[tokio::test]
    async fn test_layer_with_layer_fn() {
        let s = service_fn(|req| async move {
            println!("2-->process request {:?}", req);
            Ok::<_, Infallible>(req)
        });

        let log_layer = layer_fn(|service| LogService {
            inner: service,
            target: "test_layer_with_layer_fn",
        });

        let mut outer = log_layer.layer(s);

        let ret = outer.call("Hello").await.unwrap();
        println!("Result:{ret}");
    }
}
