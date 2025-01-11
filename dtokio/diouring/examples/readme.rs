//!
//! https://github.com/tokio-rs/io-uring/blob/master/examples/readme.rs

use std::{
    fs,
    os::unix::io::AsRawFd,
};

use io_uring::{
    opcode,
    types,
    IoUring,
};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建一个ioring实例
    let mut ring = IoUring::new(8)?;

    // 打开文件，执行任务期间确认不能关闭
    let file = fs::File::open("README.md")?;

    // 用于存储读取结果的缓冲区
    let mut buf = vec![0; 16];

    // 创建一个iouring的Read任务
    // 1.设置目标文件句柄，类型types::Fd。
    // 2.设置保存读取结果的缓冲区，类型*mut [u8]
    // 3.设置user_data回传字段值,会通过cqe回传,类型u64
    let sqe_read = opcode::Read::new(
        types::Fd(file.as_raw_fd()),
        buf.as_mut_ptr(),
        buf.len() as _,
    )
    .build()
    .user_data(0x42);

    // 任务追加到提交队列sq
    // 安全: 用户必须保证提交的任务是有效任务
    unsafe {
        ring.submission()
            .push(&sqe_read)
            .expect("submission queue is full");
    }

    // 通知内核消费任务,同时阻塞等待至少有一个任务完成
    ring.submit_and_wait(1)?;

    // 读取任务完成队列cq
    let cqe = ring
        .completion()
        .next()
        .expect("completion queue is empty");

    let content = String::from_utf8(buf.clone())?;

    println!("{:?}", content);
    println!("{:?}", cqe);

    Ok(())
}
