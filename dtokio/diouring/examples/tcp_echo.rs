//!
//! https://github.com/tokio-rs/io-uring/blob/master/examples/tcp_echo.rs

use std::{
    collections::VecDeque,
    io,
    net::TcpListener,
    os::fd::{
        AsRawFd,
        RawFd,
    },
    ptr,
};

use io_uring::{
    opcode::{
        self,
    },
    squeue,
    types,
    IoUring,
    SubmissionQueue,
};
use slab::Slab;

//
#[derive(Clone, Debug)]
enum Token {
    Accept,
    Poll {
        fd: RawFd,
    },
    Read {
        fd: RawFd,
        buf_index: usize,
    },
    Write {
        fd: RawFd,
        buf_index: usize,
        offset: usize,
        len: usize,
    },
}

/*
 * 等待新连接的任务包装
 *
 * entry:
 * - ACCEPT任务SQE条目
 *
 * count:
 * - 飞行中的ACCEPT任务额度。
 * - 飞行中：即发布出去了ACCEPT任务但是未完成(未收到新连接请求)。
 * - 每发布一个额度减一，每完成一个额度加一。到0后不能发布新的ACCEPT任务。
 *
 */
pub struct AcceptCount {
    entry: squeue::Entry,
    count: usize,
}

impl AcceptCount {
    fn new(fd: RawFd, token: usize, count: usize) -> AcceptCount {
        AcceptCount {
            entry: opcode::Accept::new(
                types::Fd(fd),
                ptr::null_mut(),
                ptr::null_mut(),
            )
            .build()
            .user_data(token as _),
            count,
        }
    }

    fn push_to(&mut self, sq: &mut SubmissionQueue) {
        // 如果ACCEPT任务额度大于0则可以发布ACCEPT任务。
        // 每当提交一个新的ACCEPT任务额度减一。
        while self.count > 0 {
            unsafe {
                match sq.push(&self.entry) {
                    Ok(_) => self.count -= 1,
                    Err(_) => break,
                }
            }
        }

        // 将rust中sqe同步到内核io_uring的sq中
        sq.sync();
    }
}

fn main() -> anyhow::Result<()> {
    let mut ring = IoUring::new(256)?;
    let listener = TcpListener::bind(("0.0.0.0", 3456))?;

    // 失败任务队列,执行失败的任务都存在这个队列中
    let mut backlog = VecDeque::new();

    // 可用内存块池.
    // 存储可用内存块的编号的列表,编号即为slab槽位的key.
    // 当需要使用内存块时,比如作为读操作的缓存:
    // 1.如果bufpool不为空,则从弹出一个编号再用编号作为key找到slab中找到这块内存的地址
    // 2.如果bufpool为空,则直接新分配一块内存,返回给申请方,同时插入slab进行管理
    // 3.如果需求方不再需要获取到的内存块,则将内存块编号归还(写回)bufpool
    let mut bufpool = Vec::with_capacity(64);
    // 内存块管理器,所有分配的内存块由slab进行管理,存储指向内存块的智能指针
    let mut buf_alloc = Slab::with_capacity(64);

    // 连接共享池
    // 存储各连接在连接存活期间持有的信息,每个连接占用一个数据槽位.通过user_data在透传的token_index进行读取.
    // 这里存储的本连接最后一次提交任务的一些信息.当任务完成时即读取到CQE时,可以从slab中用token_index取出.
    // 1.提交任务SQE时,将本次任务信息存入slab返回对应槽位的编号token_index,将槽位编号设为user_data
    // 2.读取已完成任务CQE时,再从user_data获取到槽位编号token_index,到slab中反查到对应的提交任务的信息
    //
    // user_data透传编号
    // 先将任务信息存入slab,slab会返回一个用于查询槽位的key编号,将此编号作为user_data透传编号.
    let mut token_alloc = Slab::with_capacity(64);

    println!("listen {}", listener.local_addr()?);

    // 从iouring实例获取提交句柄,SQ句柄,CQ句柄
    let (submitter, mut sq, mut cq) = ring.split();

    // AcceptCount类型包含一个ACCEPT任务和一个总连接计数值.
    // 同时还提供了一个将自身包含的ACCEPT任务提交到SQ的方法.
    let mut accept = AcceptCount::new(
        listener.as_raw_fd(),
        token_alloc.insert(Token::Accept),
        3,
    );

    // 在监听SOCKET上发布一个ACCEPT任务
    // - 每发布一个ACCEPT任务，对应的总连接数配额就会减一。
    // - 当配合变为0后，则不再发布ACCEPT任务，直到有配额。
    println!("push accept, count={}", accept.count);
    accept.push_to(&mut sq);

    loop {
        // 通知io_uring有新任务,并阻塞等待至少有一个任务完成。
        // 全局完成的首个任务应该是首个发布的ACCEPT任务。
        match submitter.submit_and_wait(1) {
            Ok(_) => (),
            Err(ref err) if err.raw_os_error() == Some(libc::EBUSY) => (),
            Err(err) => return Err(err.into()),
        }

        // 从内核io_uring的cq中同步cqe到rust中,清空rust中的历史缓存
        cq.sync();

        // 将失败的任务全部重新发布一次
        // 终止条件：
        // - 失败任务清零。
        // - SQ队列满且提交内核失败。
        loop {
            if sq.is_full() {
                match submitter.submit() {
                    Ok(_) => (),
                    Err(ref err) if err.raw_os_error() == Some(libc::EBUSY) => {
                        break
                    }
                    Err(err) => return Err(err.into()),
                }
            }

            sq.sync();

            match backlog.pop_front() {
                Some(sqe) => unsafe {
                    let _ = sq.push(&sqe);
                },
                None => break,
            }
        }

        // 提交一个ACCEPT任务
        println!("push accept, count={}", accept.count);
        accept.push_to(&mut sq);

        // 循环处理CQ中的已完成任务
        for cqe in &mut cq {
            // 获取任务结果,对不同类型的任务结果意义不同
            let ret = cqe.result();

            // 获取user_data透传的token_index.
            // 同一个连接token_index始终相同.始终对应同一个token_alloc槽位,可看做本连接的全局信息.
            let token_index = cqe.user_data() as usize;

            // 当本任务执行结果是失败的,则直接遍历下一个任务.
            if ret < 0 {
                eprintln!(
                    "token {:?} error: {:?}",
                    token_alloc.get(token_index),
                    io::Error::from_raw_os_error(-ret)
                );
                continue;
            }

            // 每发布一个任务就将任务原始信息缓存到内存池中，并返回一个查询索引。
            // 每当有任务完成，就通过回传的查询索引，找出任务的原始信息。
            let token = &mut token_alloc[token_index];

            // 根据本任务对应的token,从内存池读取原始任务信息进行后续处理.
            match token.clone() {
                Token::Accept => {
                    println!("accepted");
                    // 飞行任务额度增一
                    accept.count += 1;
                    // 对于ACCEPT类型任务,返回值为新连接的句柄
                    let fd = ret;

                    // 发布一个POLLADD任务
                    // - 关注事件：POLLIN，即连接上有数据到达(可读)，则任务完成。
                    // - POLLADD，默认为单次模式，即一个任务对应一个CQE。
                    // - POLLADD的多次模式 只需发布一次POLLADD,主要有数据到达就可以产生CQE。
                    // - 最后一个CQE：CQE的IORING_CQE_F_MORE标志本任务后续没有更多的CQE了。
                    let poll_token = token_alloc.insert(Token::Poll { fd });
                    let poll_e =
                        opcode::PollAdd::new(types::Fd(fd), libc::POLLIN as _)
                            .build()
                            .user_data(poll_token as _);

                    // 发布任务
                    // - 如果发布失败，则放入后备队列等待重新发布。
                    unsafe {
                        if sq
                            .push(&poll_e)
                            .is_err()
                        {
                            backlog.push_back(poll_e);
                        }
                    }
                }
                Token::Poll { fd } => {
                    println!("data coming");
                    // 获取一个用于存读出数据的内存块
                    // 1.如果bufpoll有内存块索引,则弹出内存块索引,并从slab中取出对应内存块
                    // 2.如果bufpoll是空的,则新分配一个内存块,纳入slab管理
                    let (buf_index, buf) = match bufpool.pop() {
                        Some(buf_index) => (buf_index, &mut buf_alloc[buf_index]),
                        None => {
                            // 创建一个长度为2048字节的内存块并转为盒子
                            let buf = vec![0u8; 2048].into_boxed_slice();
                            // 从内存管理器中获取一个空闲槽位
                            let buf_entry = buf_alloc.vacant_entry();
                            // 获取槽位对应的索引,后续可用于从内存管理器中取出对应内存块(的智能指针)
                            let buf_index = buf_entry.key();
                            // 将内存块(的智能指针)移入内存管理器的槽位中
                            (buf_index, buf_entry.insert(buf))
                        }
                    };

                    // 将token修改为本次提交的任务信息
                    *token = Token::Read { fd, buf_index };

                    // 创建一个Read任务SQE
                    // 1.fd: 读取的目标描述符
                    // 2.buf: 读取结果在用户空间的缓冲区
                    // 3.len: 用户空间缓冲区的长度
                    let read_e = opcode::Recv::new(
                        types::Fd(fd),
                        buf.as_mut_ptr(),
                        buf.len() as _,
                    )
                    .build()
                    .user_data(token_index as _);

                    // 将SQE提交到SQ队列,提交失败则写入backlog进行备份
                    unsafe {
                        if sq
                            .push(&read_e)
                            .is_err()
                        {
                            backlog.push_back(read_e);
                        }
                    }
                }
                Token::Read { fd, buf_index } => {
                    println!("readed");
                    // 返回值为0表明对端主动断开连接
                    // 1.对端发起FIN:     ESTABLISH -> FIN-WAIT-1
                    // 2.本端返回FIN ACK: ESTABLISH -> CLOSE_WAIT
                    // 3.对端收到ACK:     FIN-WAIT-1 -> FIN-WAIT-2
                    if ret == 0 {
                        // 内存块返还给内存池
                        bufpool.push(buf_index);
                        // 移除缓存的token
                        token_alloc.remove(token_index);
                        println!("shutdown");
                        // 关闭文件描述符,进而关闭本端连接
                        // 1.本端发起FIN:     CLOSE_WAIT -> LAST-ACK
                        // 2.对端发起FIN ACK: FIN-WAIT-2 -> TIME-WAIT
                        // TIME-WAIT作用:
                        // 1.如果对端没有收到FIN ACK会重发FIN
                        // 2.网络上与本链接相关的包都超时,避免影响后续链接
                        unsafe {
                            libc::close(fd);
                        }
                    } else {
                        // 读取的长度
                        let len = ret as usize;
                        // 读取缓冲块
                        let buf = &buf_alloc[buf_index];
                        // 更新Token信息
                        // len: 虽然指定发送长度为len,但是最终执行发送的时候不一定就发送了len
                        // offset: 紧跟在Read任务后创建的Write任务等于首次Write偏移量为0
                        *token = Token::Write {
                            fd,
                            buf_index,
                            len,
                            offset: 0,
                        };

                        // 创建发送任务,实现echo
                        let write_e = opcode::Send::new(
                            types::Fd(fd),
                            buf.as_ptr(),
                            len as _,
                        )
                        .build()
                        .user_data(token_index as _);

                        // 提交任务
                        unsafe {
                            if sq
                                .push(&write_e)
                                .is_err()
                            {
                                backlog.push_back(write_e);
                            }
                        }
                    }
                }
                Token::Write {
                    fd,
                    buf_index,
                    offset,
                    len,
                } => {
                    println!("writed");
                    // 实际写入的长度
                    // 1.如果小于完整长度len则还需要再次提交写入任务写剩下的
                    // 2.如果大于等于完整长度len则需要提交监听入栈数据任务,开始新的echo逻辑
                    let write_len = ret as usize;

                    // 判断上次是否写完,来创建下次任务
                    let entry = if offset + write_len >= len {
                        bufpool.push(buf_index);
                        *token = Token::Poll { fd };
                        opcode::PollAdd::new(types::Fd(fd), libc::POLLIN as _)
                            .build()
                            .user_data(token_index as _)
                    } else {
                        // buf中剩余数据的偏移量
                        let offset = offset + len;
                        // buf中剩余数据的长度
                        let len = len - write_len;
                        // buf缓冲块
                        let buf = &buf_alloc[buf_index];

                        // 上次没写完,还得再提交写任务再写一次
                        *token = Token::Write {
                            fd,
                            buf_index,
                            len,
                            offset,
                        };
                        opcode::Send::new(types::Fd(fd), buf.as_ptr(), len as _)
                            .build()
                            .user_data(token_index as _)
                    };

                    unsafe {
                        if sq
                            .push(&entry)
                            .is_err()
                        {
                            backlog.push_back(entry);
                        }
                    }
                }
            }
        }
    }
}
