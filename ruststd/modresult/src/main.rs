fn main() {
    let result: Result<i32, i32> = Ok(100);

    let r = result.map(|x| x.to_string());
    let r = result.map_err(|e| e.to_string());
}
