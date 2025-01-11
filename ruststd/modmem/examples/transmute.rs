use core::slice;
use std::mem::transmute;

fn main() {}

fn split_as_mut_transmute<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(mid <= len);

    unsafe {
        let slice2 = transmute::<&mut [T], &mut [T]>(slice);
        (&mut slice[0..mid], &mut slice2[mid..len])
    }
}
fn split_as_mut_casts<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(mid <= len);

    unsafe {
        let slice2 = &mut *(slice as *mut [T]);
        (&mut slice[0..mid], &mut slice2[mid..len])
    }
}

fn split_as_mut_stdlib<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    assert!(mid <= len);

    unsafe {
        let ptr = slice.as_mut_ptr();
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len),
        )
    }
}
