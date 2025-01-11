fn main() {
    // 只为相同元素类型的元组实现了From
    let t = (1i32, 2i32, 3i32);
    let array = <[i32; 3]>::from(t);

    println!("{:?}", array);
}
