#![allow(unused_imports)]

// == methods ===
// and_then
// boxed
// boxed_clone
// call_all
// filter
// filter_async
// map_err
// map_future
// map_request
// map_response
// map_result
// oneshot
// ready
// ready_and
// ready_oneshot
// then
// == methods ===

use tokio;
use tower::{BoxError, Service, ServiceExt};

use std::error::Error;
use std::fmt::Debug;
use std::future::{ready, Future, Ready};
use std::ops::Mul;
use std::task::Poll;

#[tokio::main]
async fn main() {}

struct FirstLetterService;

impl Service<&'static str> for FirstLetterService {
    type Response = &'static str;
    type Error = BoxError;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: &'static str) -> Self::Future {
        ready(Ok(&req[..1]))
    }
}

#[cfg(test)]
mod test {
    use std::{
        error::Error,
        fmt::{Display, Write},
    };

    use crate::FirstLetterService;
    use futures::{channel::mpsc::unbounded, StreamExt};
    use tower::{util::BoxService, BoxError, Service, ServiceExt};

    #[tokio::test]
    async fn test_service() {
        let mut s = FirstLetterService;
        let ret = s.ready().await.unwrap().call("Hello,World").await.unwrap();
        assert_eq!(ret, "H");
    }

    #[tokio::test]
    async fn test_call_all() {
        let (reqs, rx) = unbounded();
        _ = reqs.unbounded_send("Hello");
        _ = reqs.unbounded_send("Rust");
        _ = reqs.unbounded_send("World");

        let mut new_service = FirstLetterService.call_all(rx);
        drop(reqs);

        let mut result = vec![];
        while let Some(rsp) = new_service.next().await {
            result.push(rsp.unwrap());
        }

        assert_eq!(result, vec!["H", "R", "W"]);
    }

    #[tokio::test]
    async fn test_call_all_unordered() {
        let (reqs, rx) = unbounded();
        _ = reqs.unbounded_send("Hello");
        _ = reqs.unbounded_send("Rust");
        _ = reqs.unbounded_send("World");

        let mut new_service = FirstLetterService.call_all(rx).unordered();
        drop(reqs);

        let mut result = vec![];
        while let Some(rsp) = new_service.next().await {
            result.push(rsp.unwrap());
        }

        assert_eq!(result, vec!["H", "R", "W"]);
    }

    #[tokio::test]
    async fn test_filter() {
        type Result<T> = std::result::Result<T, BoxError>;

        #[derive(Debug)]
        struct ContainError;
        impl Error for ContainError {}
        impl Display for ContainError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("ContainError")
            }
        }

        let mut new_service = FirstLetterService.filter(|request: &'static str| {
            if request.contains("He") {
                Result::Ok(request)
            } else {
                Err(Box::new(ContainError) as BoxError)
            }
        });

        let ret = new_service
            .ready()
            .await
            .unwrap()
            .call("ello00")
            .await
            .unwrap_or("AA");
        assert_eq!(ret, "AA");

        let ret = new_service
            .ready()
            .await
            .unwrap()
            .call("Hello")
            .await
            .unwrap_or("AA");
        assert_eq!(ret, "H");
    }
}
