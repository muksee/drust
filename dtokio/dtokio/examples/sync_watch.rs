fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::{sync::watch, time};

    #[tokio::test]
    async fn channel() {
        let (tx, mut rx) = watch::channel(100);
        tokio::spawn(async move {
            while rx.changed().await.is_ok() {
                println!("Received {:?}", *rx.borrow());
            }
        });
        let _ = tx.send(1000);
        time::sleep(Duration::from_secs(2)).await;
    }
}
