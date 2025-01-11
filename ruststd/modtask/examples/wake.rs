use std::{
    future::Future,
    pin::pin,
    sync::Arc,
    task::{
        Context,
        Poll,
        Wake,
    },
    thread::{
        self,
        Thread,
    },
};

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: std::sync::Arc<Self>) {
        self.0
            .unpark();
    }
}

fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = pin!(fut);

    let t = thread::current();
    let waker = Arc::new(ThreadWaker(t)).into();
    let mut ctx = Context::from_waker(&waker);

    loop {
        match fut
            .as_mut()
            .poll(&mut ctx)
        {
            Poll::Ready(result) => return result,
            Poll::Pending => thread::park(),
        }
    }
}

fn main() {
    block_on(async {
        println!("Hi from inside a future!");
    })
}
