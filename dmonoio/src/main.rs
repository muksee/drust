use monoio::fs::File;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[monoio::main]
async fn main() -> Result {
    let file = File::create("./hello.txt").await?;

    let (res, buf) = file
        .write_at(&b"hello world"[..], 0)
        .await;
    let n = res?;

    println!("write {} bytes", n);

    file.sync_all()
        .await?;
    file.close()
        .await?;

    Ok(())
}
