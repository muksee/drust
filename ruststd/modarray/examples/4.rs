#![feature(array_try_map)]
fn main() {
    let array = ["1", "2", "3"];
    // let array = ["1", "2a", "3"];
    println!("{array:?}");
    let new = array
        .try_map(|s| s.parse::<i32>())
        .unwrap()
        .map(|x| x * x);

    println!("{new:?}");
}
