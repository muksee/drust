use std::time::Duration;

use smol::{
    future,
    Timer,
};

fn main() {
    // let ex = Executor::new();

    let task = smol::spawn(async {
        loop {
            println!("i am a endless task");
            Timer::after(Duration::from_secs(2)).await;
        }
    });

    future::block_on(async {
        Timer::after(Duration::from_secs(3)).await;
        task.cancel().await;
        Timer::after(Duration::from_secs(10)).await;
    })
}
