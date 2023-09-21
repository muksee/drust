use std::vec;

use futures::{
    stream,
    Stream,
    StreamExt,
};

#[tokio::main]
async fn main() {
    let mut v = vec![];

    for _ in 0..10 {
        v.push(generate_stream());
    }

    
}

async fn generate_stream() -> impl Stream {
    stream::iter(1..=3)
}
