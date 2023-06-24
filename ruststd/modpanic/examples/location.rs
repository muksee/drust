use std::panic::Location;

fn main() {
    let fixed_location = get_just_one_location();
    println!(
        "fixed_location: {},{},{}",
        file!(),
        fixed_location.line(),
        fixed_location.column()
    );

    let second_fixed_location = get_just_one_location();
    println!(
        "fixed_location: {},{},{}",
        file!(),
        second_fixed_location.line(),
        second_fixed_location.column()
    );

    let this_location = get_caller_location();
    println!(
        "fixed_location: {},{},{}",
        file!(),
        this_location.line(),
        this_location.column()
    );

    let another_location = get_caller_location();
    println!(
        "fixed_location: {},{},{}",
        file!(),
        another_location.line(),
        another_location.column()
    );
}

// trace_caller属性:
// 被trace_caller修饰的函数被调用,获取代码位置时,
#[track_caller]
fn get_caller_location() -> &'static Location<'static> {
    Location::caller()
}

// 这个函数被调用时,获取到的位置永远是相同的,本函数的位置
fn get_just_one_location() -> &'static Location<'static> {
    get_caller_location()
}
