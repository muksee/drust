use futures::future::FutureExt;
use futures::future::TryFuture;
use futures::future::TryFutureExt;
use std::time::Duration;
use tokio;
use tokio::time;

#[tokio::main]
async fn main() {
    let fut = async {
        time::sleep(Duration::from_secs(5)).await;
        println!("task 1 complete");
        Err::<i32, i32>(100)
    };

    let ret = fut.map_err(|x| x ^ 2).await;

    println!("result: {:?}", ret);
}
