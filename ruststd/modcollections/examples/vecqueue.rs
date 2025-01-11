use std::collections::VecDeque;

fn main() {
    let mut v: VecDeque<i32> = (0..=10)
        .into_iter()
        .collect();

    v.extend(6..10);
    let (a, b) = v.as_slices();

    println!("{a:?}, {b:?}");
}
