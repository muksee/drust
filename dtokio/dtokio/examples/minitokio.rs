use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Context;
use std::task::Poll;
use std::thread;
use std::time;
use std::time::Duration;
use std::time::Instant;

use futures::task;
use futures::task::ArcWake;
use futures::task::Waker;

use crossbeam::channel;

fn main() {
    let mut mini_tokio = MiniTokio::new();

    for i in 3..8 {
        println!("==== Start ====");
        mini_tokio.spawn(async move {
            let delay = Delay {
                when: time::Instant::now() + Duration::from_secs(i),
                waker: None,
            };
            println!("Hello wait,{}", i);
            delay.await;
            println!("Hello,end, {}", i);
        });
    }

    mini_tokio.run();
}

struct MiniTokio {
    sender: channel::Sender<Arc<Task>>,
    scheduled: channel::Receiver<Arc<Task>>,
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { sender, scheduled }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });
        let _ = sender.send(task);
    }

    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }

    fn poll(self: &Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

struct Delay {
    when: time::Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        println!("Future in");
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        println!("Future out");

        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
