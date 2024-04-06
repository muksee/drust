use tokio::sync::{
    mpsc,
    oneshot,
};

#[tokio::main]
async fn main() {
    let handle = MyActorHandle::new();
    for _ in 0..10 {
        let id = handle
            .get_unique_id()
            .await;
        println!("Get id: {id}");
    }
}

struct MyActor {
    msg_box: mpsc::Receiver<MyActorMessage>,
    next_id: usize,
}

impl MyActor {
    /// 方式一: 方法
    async fn run(&mut self) {
        println!("Start MyActor task");
        while let Some(msg) = self
            .msg_box
            .recv()
            .await
        {
            self.handle_message(msg);
        }
    }

    /// 方式二: 函数
    #[allow(unused)]
    async fn run_my_actor(mut actor: MyActor) {
        while let Some(msg) = actor
            .msg_box
            .recv()
            .await
        {
            actor.handle_message(msg);
        }
    }

    fn new(msg_box: mpsc::Receiver<MyActorMessage>) -> MyActor {
        MyActor {
            msg_box,
            next_id: 0,
        }
    }

    fn handle_message(&mut self, message: MyActorMessage) {
        println!("MyActor: handle new request");
        match message {
            MyActorMessage::GetUniqueId { response_to } => {
                self.next_id += 1;
                let _ = response_to.send(self.next_id);
            }
        }
    }
}

/// actor接收的请求消息
enum MyActorMessage {
    GetUniqueId { response_to: oneshot::Sender<usize> },
}

/// 句柄的作用
/// - 创建actor,handle,启动actor.
/// - 向actor发起业务请求.
#[derive(Clone)]
struct MyActorHandle {
    sender: mpsc::Sender<MyActorMessage>,
}

impl MyActorHandle {
    fn new() -> MyActorHandle {
        let (sender, receiver) = mpsc::channel(10);
        let mut my_actor = MyActor::new(receiver);

        tokio::spawn(async move {
            my_actor
                .run()
                .await;
        });
        MyActorHandle { sender }
    }
    async fn get_unique_id(&self) -> usize {
        let (sender, receiver) = oneshot::channel();
        let _ = self
            .sender
            .send(MyActorMessage::GetUniqueId {
                response_to: sender,
            })
            .await;
        receiver
            .await
            .expect("Actor task has been killed")
    }
}
