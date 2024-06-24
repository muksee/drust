fn main() {
    let array = [1i32, 2, 3];

    let r1 = array.each_ref();
    println!("{r1:?}");

    let s = array.as_slice();
    println!("{s:?}");
    let s = &array[..];
    println!("{s:?}");
}
