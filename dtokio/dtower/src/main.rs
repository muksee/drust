#![allow(unused_imports)]
use tokio;
use tower::{Service, ServiceExt};

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
    type Error = Box<dyn Error + Send + Sync>;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: &'static str) -> Self::Future {
        ready(Ok(&req[..1]))
    }
}

#[cfg(test)]
mod tests {
    use crate::FirstLetterService;
    use tower::Service;

    #[tokio::test]
    async fn test_service() {
        let mut s = FirstLetterService;
        let ret = s.call("Hello,World").await.unwrap();
        assert_eq!(ret, "H");
    }

    #[tokio::test]
    async fn test_and_then() {}
}
