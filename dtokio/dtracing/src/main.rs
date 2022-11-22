use std::io;
use tracing::event;
use tracing::instrument;
use tracing::span;
use tracing::Level;


struct Person {
    name: &'static str,
}

fn main() {
    tracing_subscriber::fmt()
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

    span_func();
}

#[instrument]
fn span_func() {
    let answer = 50;
    let question = "life, the universe, and everything";
    event!(Level::DEBUG, answer, question);

    let p = Person {
        name: "Lucy",
    };
    event!(Level::DEBUG, p.name);

    event!(Level::DEBUG, "Something happened inside span_func 1");
    event!(Level::DEBUG, "Something happened inside span_func 1");
}
