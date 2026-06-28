#[allow(dead_code)]
#[derive(Debug)]
struct User {
    id: usize,
}

fn main() {
    // we didn't created some kind of User object and it wont give us warning because of allow(dead_code) attr
}
