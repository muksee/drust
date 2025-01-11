use lazystatic;

fn main() {
    lazystatic::lazy_static! {
        static ref FOO: String = "lazy_static".to_owned()
    }

    println!("Hello world, {}", *FOO);

    say();

}

fn say() {
    println!("say hello");
}
