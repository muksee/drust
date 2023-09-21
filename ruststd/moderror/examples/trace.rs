use std::{
    backtrace::Backtrace,
    fmt::Display,
};

#[derive(Debug)]
struct SourceError {}

impl Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "Source Error")
    }
}

#[derive(Debug)]
struct MyError {
    backtrace: Backtrace,
    source: SourceError,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(
            f,
            "My Error \n\n  caused by: \n\n  {}\n\n{}",
            self.source,
            self.backtrace
        )
    }
}

fn main() {
    let se = SourceError{};
    let me = MyError { backtrace:Backtrace::capture(), source: se};

    println!("{me}");
}
