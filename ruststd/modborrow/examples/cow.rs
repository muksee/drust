use std::borrow::Cow;

fn main() {
    let s = String::from("Hello Cow");

    let mut c = Cow::Borrowed(&s);

    println!("{s}, {c}");

    let cc = c.to_mut();

    
}
