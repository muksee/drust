use std::{
    future::Future, pin::Pin, sync::{mpsc::{channel, Sender}, Arc, Mutex},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    thread::{self, JoinHandle}, time::{Duration, Instant}
};

fn block_on<F: Future>(mut future: F) -> F::Output {
    let mywaker = Arc::new(MyWaker{thread: thread::current()});
    let waker = wake_into_waker(Arc::into_raw(mywaker));

    let mut cx = Context::from_waker(&wakder);

    let mut future = unsafe { Pin::new_unchecked(&mut future)};

    let val = loop {
        match Future::poll(future, &mut cx) {
            Poll::Ready(val) => break val,
            Poll::Pending => thread::park()
        }
    };

    val
}

#[derive(Clone)]
struct MyWaker {
    thread: thread::Thread,
}

#[derive(Clone)]
pub struct Task {
    id: isize,
    reactor: Arc<Mutex<Box<Reactor>>>,
    data: u64
}

fn main() {}