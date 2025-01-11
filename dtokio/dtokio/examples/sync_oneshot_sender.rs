fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::{sync::oneshot, time};

    #[tokio::test]
    async fn top1() {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            if let Err(_) = tx.send("Hello, onshot") {
                println!("The receiver dropped!");
            }
        });

        match rx.await {
            Ok(v) => println!("Got = {:?}", v),
            Err(_) => println!("The Sender dropped"),
        }
    }

    #[tokio::test]
    async fn top2() {
        let (tx, rx) = oneshot::channel::<u32>();

        tokio::spawn(async move { drop(tx) });

        match rx.await {
            Ok(_) => println!("This never happen"),
            Err(e) => println!("The Sender dropped {:?}", e),
        }
    }

    #[tokio::test]
    async fn top3() {
        struct SendOnDrop {
            sender: Option<oneshot::Sender<&'static str>>,
        }

        impl Drop for SendOnDrop {
            fn drop(&mut self) {
                if let Some(tx) = self.sender.take() {
                    let _ = tx.send("Hello, oneshot");
                }
            }
        }

        let (tx, rx) = oneshot::channel();

        let send_on_drop = SendOnDrop { sender: Some(tx) };
        drop(send_on_drop);

        assert_eq!(rx.await, Ok("Hello, oneshot"));
    }

    #[tokio::test]
    async fn t_closed() {
        let (mut tx, rx) = oneshot::channel::<()>();

        tokio::spawn(async move { drop(rx) });

        tx.closed().await;
        dbg!("The receiver dropped");
    }

    #[tokio::test]
    async fn t_closed_select() {
        let (mut tx, rx) = oneshot::channel();

        async fn compute() -> String {
            time::sleep(Duration::from_secs(5)).await;
            String::from("Lily")
        }

        tokio::spawn(async move {
            tokio::select! {
                _ = tx.closed() => {
                    println!("tx closed");
                }
                value = compute() => {
                    let _ = tx.send(value);
                }
            }
        });

        let _ = time::timeout(Duration::from_secs(4), rx).await;
    }
}
