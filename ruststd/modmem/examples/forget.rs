use std::mem::{self, ManuallyDrop};

fn main() {
    forget();
    manually_drop();
}

fn forget() {
    let mut v = vec![65, 122];
    let s = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };

    println!("{:?}", v);
    println!("{:?}", s);

    mem::forget(v);
}

fn manually_drop() {
    let v = vec![65, 122];
    let mut v = ManuallyDrop::new(v);
    let s = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.capacity()) };

    println!("{:?}", v);
    println!("{:?}", s);
}
