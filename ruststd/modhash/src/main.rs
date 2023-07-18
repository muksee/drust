use std::{
    collections::hash_map::DefaultHasher,
    hash::{
        Hash,
        Hasher,
    },
};

fn main() {
    println!("Hello, world!");

    let person1 = Person {
        id: 5,
        name: String::from("Lucy"),
        phone: 20,
    };

    let hash = calculate_hash(&person1);

    println!("{person1:?} -> {hash:X}");
}

#[derive(Hash, Debug)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}

fn calculate_hash(p: &Person) -> u64 {
    let mut s = DefaultHasher::default();
    p.hash(&mut s);
    s.finish()
}
