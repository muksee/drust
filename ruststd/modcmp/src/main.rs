fn main() {
    println!("Hello, world!");
}

#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height
            .cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}
