fn main() {
    let mut array = ["a", "b", "c", "d"];

    for s in &array {
        println!("{s}");
    }

    for s in &mut array {
        *s = "nnn"
    }
    println!("{array:?}");

    for s in array {
        println!("{s}");
    }
}
