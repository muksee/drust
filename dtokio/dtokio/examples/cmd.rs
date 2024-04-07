use std::{
    process::Stdio,
    time::Duration,
};

use tokio::{
    io::{
        AsyncReadExt,
        BufReader,
    },
    process::Command,
    time,
};

#[tokio::main]
async fn main() {
    let mut child = Command::new("ping")
        .arg("github.com")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child
        .stdout
        .take()
        .unwrap();
    let _stderr = child
        .stderr
        .take()
        .unwrap();

    tokio::spawn(async move {
        let mut bufreader = BufReader::new(stdout);
        let mut buf = [0; 2048];
        while let Ok(n) = bufreader
            .read(&mut buf)
            .await
        {
            print!("{}", String::from_utf8_lossy(&buf[..n]));
        }
    });

    time::sleep(Duration::from_secs(1000)).await;
}
