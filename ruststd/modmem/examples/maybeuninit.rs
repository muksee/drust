use std::{
    mem,
    mem::MaybeUninit,
    ptr::addr_of_mut,
};

fn main() {
    demo_init_array_element_by_element();
    demo_init_struct_field_by_field();
}

fn demo() {
    let mut x = MaybeUninit::<&i32>::uninit();
    x.write(&0);
    let x = unsafe { x.assume_init() };
    println!("{x:p}");
}

fn demo_out_pointer() {
    let mut v = MaybeUninit::uninit();
    unsafe {
        make_vec(v.as_mut_ptr());
    };
    let v = unsafe { v.assume_init() };
    println!("{v:?}");
}

unsafe fn make_vec(out: *mut Vec<i32>) {
    out.write(vec![1, 2, 3]);
}

fn demo_init_array_element_by_element() {
    let data = {
        let mut data: [MaybeUninit<Vec<u32>>; 1000] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for e in &mut data[..] {
            let x = e.write(vec![42, 42]);
        }

        unsafe { mem::transmute::<_, [Vec<u32>; 1000]>(data) }
    };

    println!("{:?}", &data[0]);
}

#[derive(Debug, PartialEq)]
struct Foo {
    name: String,
    list: Vec<u8>,
}

fn demo_init_struct_field_by_field() {
    let mut f = MaybeUninit::<Foo>::uninit();
    let p = f.as_mut_ptr();

    unsafe { addr_of_mut!((*p).name).write("Bob".to_owned()) };
    unsafe { addr_of_mut!((*p).list).write(vec![42, 42]) };

    let f = unsafe { f.assume_init() };

    println!("{f:?}");
}
