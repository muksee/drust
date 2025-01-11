#![allow(unused)]
fn main() {}

#[cfg(test)]
mod tests {
    use std::ptr::*;

    #[test]
    fn as_ref() {
        let mut x = 0u32;
        let ptr: NonNull<u32> = NonNull::new(&mut x as *mut _).expect("ptr is null");
        let rx = unsafe { ptr.as_ref() };
        assert_eq!(*rx, 0);
    }
    #[test]
    fn as_mut() {
        let mut x = 0u32;
        let mut ptr: NonNull<u32> = NonNull::new(&mut x as *mut _).expect("ptr is null");
        let rx = unsafe { ptr.as_mut() };
        *rx = 2;
        assert_eq!(*rx, 2);
    }

    #[test]
    fn cast() {
        let mut x = 0u32;
        let ptr: NonNull<u32> = NonNull::new(&mut x as *mut _).expect("ptr is null");
        let ptr_i8 = ptr.cast::<i8>();
        assert_eq!(*unsafe { ptr_i8.as_ref() }, 0);
    }
}
