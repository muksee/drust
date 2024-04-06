use futures::{
    executor,
    stream,
    StreamExt,
};

fn main() {
    // 执行异步任务的线程池
    // let pool = ThreadPool::new().expect("Create ThreadPool Failed");

    // 异步根任务
    let root_task = async {
        let stream = stream::iter(0..10);
        stream
            .collect::<Vec<u32>>()
            .await
    };

    // 执行器启动根任务
    let ret = executor::block_on(root_task);

    println!("{ret:?}");
}
