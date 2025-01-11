use libc;

fn main() {
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        eprintln!("错误！");
    } else if pid == 0 {
        println!("子进程空间");
    } else {
        println!("父进程空间, 子进程 pid 为 {}", pid);
    }
}
