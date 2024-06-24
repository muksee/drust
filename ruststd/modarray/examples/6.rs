fn main() {
    let array = [1i32, 2, 3];
    let first = array[0];
    let [.., second, third] = array;

    println!("{first},{second}, {third}");
}
