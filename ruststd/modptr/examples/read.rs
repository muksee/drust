#![allow(unused)]
use std::ptr;

#[repr(packed, C)]
struct Packed {
    _padding: u8,
    unaligned: u32,
}

fn main() {
    let mut s = String::from("foo");
    let s2 = ownership(&mut s);

    assert_eq!(s, "bar");
    assert_eq!(s2, "");
}

fn swap<T>(a: &mut T, b: &mut T) {
    unsafe {
        let temp = ptr::read(a);
        ptr::copy_nonoverlapping(b, a, 1);
        ptr::write(b, temp);
    }
}

fn ownership(s: &mut String) -> String {
    unsafe {
        let mut s2: String = ptr::read(s);
        s2 = String::default();
        println!("s2={:?}", s2);
        println!("s={:?}", s); //在main中和test中结果不同?why?

        ptr::write(s, String::from("bar"));
        s2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let mut a = "foo".to_owned();
        let mut b = "bar".to_owned();
        swap(&mut a, &mut b);
        assert_eq!(a, "bar");
        assert_eq!(b, "foo");
    }
    #[test]
    fn test_ownership() {
        let mut s = String::from("foo");
        let s2 = ownership(&mut s);

        assert_eq!(s, "bar");
        assert_eq!(s2, "");
    }
}
