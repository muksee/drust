fn main() {
    println!("hello, world");

    extra();

    panic!("Hello");
}

#[cfg(feature = "func")]
fn extra() {
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
    println!("hello, world");
}
