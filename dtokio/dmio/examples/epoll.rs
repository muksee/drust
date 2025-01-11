use std::io;
use std::os::unix::io::RawFd;

#[macro_export]
macro_rules! syscall {
    ($fn:ident ($($arg:expr),* $(,)*)) => {{
        let ret = unsafe { libc::$fn($($arg,)*)};
        if ret == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret)
        }
}};
}
fn main() {}
