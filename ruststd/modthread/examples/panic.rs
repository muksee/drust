use std::thread;

fn main() {
    let jh1 = thread::spawn(|| {
        panic!("i am panic in thread");
    });

    let jh2 = thread::spawn(|| {
        return 100i32;
    });

    let r1 = jh1.join();
    let r2 = jh2.join();

    println!("{r1:?},{r2:?}");
}
