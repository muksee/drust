use std::{convert::Infallible, ops::Mul};

use tokio;

use tower::service_fn;
use tower::Service;

#[tokio::main]
async fn main() {
    let mut s = service_fn(power);
    let ret = s.call(100).await.unwrap();

    println!("{ret}");
}

async fn power<T>(x: T) -> Result<T, Infallible>
where
    T: Mul<Output = T> + Copy,
{
    return Ok(x * x);
}
