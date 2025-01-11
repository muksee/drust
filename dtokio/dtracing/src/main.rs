use std::{
    error::Error,
    fmt::Display,
    io,
};
use tracing::{
    event,
    instrument,
    span,
    Level,
};

struct Person {
    name: &'static str,
}

fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_writer(io::stderr)
        .with_max_level(Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::time())
        .init();

    event!(Level::INFO, "Somthing happend before my_span");

    let user = "Lucy";
    let span = span!(Level::INFO, "my_span", user);
    let _guard = span.enter();
    event!(Level::DEBUG, "Something happened inside my_span 1");
    event!(Level::DEBUG, "Something happened inside my_span 2");
    drop(_guard);

    event!(Level::DEBUG, "Something happened after my_span");

    let _ = span_func("name", 100);
}

#[instrument(ret, err, target = "====================")]
pub fn span_func(name: &str, age: i32) -> Result<String, MyError> {
    let answer = 50;
    let question = "life, the universe, and everything";
    event!(Level::DEBUG, answer, question);

    let p = Person { name: "Lucy" };
    event!(Level::DEBUG, p.name);

    event!(Level::DEBUG, "Something happened inside span_func 1");
    event!(Level::DEBUG, "Something happened inside span_func 1");

    // Ok(String::from("i am return value"))
    Err(MyError)
}

#[derive(Debug)]
pub struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("i am from error display")
    }
}

impl Error for MyError {}

#[test]
fn my_test() {
    println!("Hello world");
}
