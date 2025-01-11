#![allow(unused)]

use std::mem;
use std::ptr;
use std::ptr::copy;

fn main() {
    let src = Box::new(100);
    let mut dst = Box::new(200);

    let src_ptr = &src as *const Box<i32>;
    let dst_ptr = (&mut dst) as *mut Box<i32>;

    println!(
        "{:?}, {:?}",
        mem::size_of::<&Box<i32>>(),
        mem::size_of::<&mut Box<i32>>()
    );

    unsafe {
        println!("{:?},{:?}", *src_ptr, *dst_ptr);
    }
}

// copy
fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);
    unsafe {
        ptr::copy(ptr, dst.as_mut_ptr(), elts);
        dst.set_len(elts);
    }
    dst
}

// copy_nonoverlapping
fn append<T>(src: &mut Vec<T>, dst: &mut Vec<T>) {
    let src_len = src.len();
    let dst_len = dst.len();

    dst.reserve(src_len);

    unsafe {
        let dst_ptr = dst.as_mut_ptr().add(dst_len);
        let src_ptr = src.as_ptr();

        src.set_len(0);

        ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);

        dst.set_len(src_len + dst_len)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;
    use ptr::*;

    #[test]
    fn test_copy() {
        let mut v1 = vec!['r', 's'];
        let mut v2 = vec!['a', 'b', 'c'];

        append(&mut v1, &mut v2);

        assert_eq!(v1, vec![]);
        assert_eq!(v2, vec!['a', 'b', 'c', 'r', 's'])
    }

    #[test]
    fn test_copy_nonoverlapping() {
        let src = [10u32; 4];
        let v = from_buf_raw(src.as_ptr(), src.len());

        assert_eq!(v, vec![10u32; 4])
    }

    #[test]
    fn test_leak() {
        let src = Box::new(100);
        let mut dst = Box::new(200);

        let src_ptr = &src as *const Box<i32>;
        let dst_ptr = &mut dst as *mut Box<i32>;

        println!(
            "src_ptr={:x},dst_ptr={:x}",
            src_ptr as usize, dst_ptr as usize
        );
        println!("*src={:?},*dst={:?}", *src, *dst);

        println!("align={}", mem::align_of::<Box<i32>>());

        unsafe {
            copy(src_ptr, dst_ptr, 1);
            mem::forget(src);
        }
        assert_eq!(*dst, 100);
    }
}
