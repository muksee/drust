use std::{
    thread,
    time::Duration,
};

fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        let jh1 = s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        let jh2 = s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
            thread::sleep(Duration::from_secs(10));
        });
        println!("hello from the main thread");
    });

    println!("hello after scope");

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}
