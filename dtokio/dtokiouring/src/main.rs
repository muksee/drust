use tokio_uring::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio_uring::start(async {
        let file = File::open("Cargo.toml").await?;
        let buf = vec![0; 4096];
        let (res, buf) = file
            .read_at(buf, 0)
            .await;
        let n = res?;

        println!("{}", String::from_utf8(buf.clone())?);

        Ok(())
    })
}
