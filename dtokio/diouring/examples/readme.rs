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

    // 获取文件描述符,但文件不能关闭
    // let file = fs::File::open("README.md")?;
    // let fd = file.as_raw_fd();

    // 这个方法也能获取描述符,但是会导致File被遗弃文件关闭
    let fd = fs::File::open("README.md").map(|f| f.as_raw_fd())?;

    // 用于存储读取结果的缓冲区
    let mut buf = vec![0; 1024];

    // 创建一个iouring的任务,设置user_data回传字段值,会回传给对应的cqe
    let sqe_read =
        opcode::Read::new(types::Fd(fd), buf.as_mut_ptr(), buf.len() as _)
            .build()
            .user_data(0x42);

    // 任务追加到提交队列sq
    // 安全: 用户必须保证提交的任务是有效任务
    unsafe {
        ring.submission()
            .push(&sqe_read)
            .expect("submission queue is full");
    }

    // 通知内核iouring有新进任务,同时阻塞等待至少有一个任务完成
    ring.submit_and_wait(1)?;

    // 读取任务完成队列cq
    let cqe = ring
        .completion()
        .next()
        .expect("completion queue is empty");

    let content = String::from_utf8(buf)?;

    println!("{:?}", content);
    println!("{:?}", cqe.result());

    Ok(())
}
