use std::time::Duration;

use tokio::{self, time::Instant};

#[tokio::main(flavor = "current_thread", start_paused = true)]
async fn main() {
    let t1 = Instant::now();

    for _ in 0..100000000 {}
    println!("{:?}", Instant::now() - t1);

    tokio::time::advance(Duration::from_secs(1)).await;
    println!("{:?}", Instant::now() - t1);

    tokio::time::resume();
    for _ in 0..100000000 {}
    println!("{:?}", Instant::now() - t1);
}
