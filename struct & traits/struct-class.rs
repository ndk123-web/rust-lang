struct User {
    name: String,
    age: u8,
    rank: u32,
}

impl User {
    // static method
    fn new(name: String, age: u8, rank: u32) -> User {
        User { name, age, rank }
    }

    // method
    fn get_rank(&self) {
        println!("{}", self.rank);
    }

    // method
    fn get_name(&self) {
        println!("{}", self.name);
    }

    // method
    fn get_age(&self) {
        println!("{}", self.age);
    }

    // moves owneship
    fn destroy(self) {
        println!("Destroyedd");
    }
}

fn main() {
    let u: User = User::new(String::from("Ndk"), 12, 1);

    u.get_rank();
    u.get_age();
    u.get_name();
    u.destroy();
}
