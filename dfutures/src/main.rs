use futures::{
    channel::mpsc,
    executor::{
        self,
        ThreadPool,
    },
    StreamExt,
};

fn main() {
    // 执行异步任务的线程池
    let pool = ThreadPool::new().expect("Create ThreadPool Failed");
    // 无界队列
    let (tx, rx) = mpsc::unbounded::<u32>();

    // 异步根任务
    let root_task = async {
        // 定义发送任务
        let fut_tx_result = async move {
            (0..=10).for_each(|v| {
                let _ = tx.unbounded_send(v);
            })
        };
        // 调度发送任务
        pool.spawn_ok(fut_tx_result);

        // 接收任务
        let fut_values = rx
            .map(|v| v * 2)
            .collect();

        // 执行接收任务
        fut_values.await
    };

    // 执行器启动根任务
    let values: Vec<u32> = executor::block_on(root_task);

    println!("{values:?}");
}
