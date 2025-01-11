fn main() {}

mod test {
    #[tokio::test(start_paused = true)]
    async fn hello_test() {
        use std::time::Duration;
        use tokio::time::{
            self,
            Instant,
        };
        time::advance(Duration::from_secs(60)).await;
        let handle = tokio::spawn(async move {
            println!("sleep start {:?}", Instant::now());
            time::sleep(Duration::from_secs(20)).await;
            println!("sleep stop {:?}", Instant::now());

            let mut ticker = tokio::time::interval(Duration::from_secs(3));

            println!("tick start {:?}", Instant::now());
            ticker
                .tick()
                .await;
            ticker
                .tick()
                .await;
            ticker
                .tick()
                .await;
            ticker
                .tick()
                .await;
            ticker
                .tick()
                .await;
            println!("tick stop {:?}", Instant::now());
        });

        let _ = handle.await;
        println!("resume start {:?}", Instant::now());
        time::resume();
        time::sleep(Duration::from_secs(3)).await;
        println!("resume stop {:?}", Instant::now());
    }
}
