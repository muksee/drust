use std::ascii::escape_default;

fn main() {
    for c in escape_default(b'\t') {
        println!("{} -> {}", "\\t", c);
    }

    for c in escape_default(b'\x88') {
        println!("{} -> {}", "\\t", c);
    }
}
