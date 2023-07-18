#![feature(provide_any)]

use std::any::{
    request_ref,
    request_value,
    Provider,
};

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }
}

impl Provider for Person {
    fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {
        demand
            .provide_ref::<str>(&self.name)
            .provide_value::<u32>(self.age)
            .provide_value::<String>(String::from("on my will"));
    }
}

fn main() {
    let p = Person::new("libo", 25);

    let my_str = request_ref::<str>(&p).unwrap();
    let my_u32 = request_value::<u32>(&p).unwrap();
    let my_string = request_value::<String>(&p).unwrap();

    println!("str: {my_str}, u32: {my_u32}, my_string: {my_string}");
}
