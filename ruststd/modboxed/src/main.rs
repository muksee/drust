use std::mem::size_of;

fn main() {
    println!("Hello, world!");

    let b = Box::new(100i32);

    println!("{}", size_of::<Box<i32>>());
}
