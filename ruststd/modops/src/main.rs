#![feature(generators, generator_trait)]

use std::{
    ops::{
        Generator,
        GeneratorState,
    },
    pin::Pin,
};

fn main() {
    let mut generator = || {
        yield 1;
        "foo"
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(x) => {
            println!("yield: {x}");
        }
        _ => panic!("unexpected return from resume"),
    }

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete(x) => {
            println!("complete: {x}");
        }
        _ => panic!("unexpected return from resume"),
    }
}
