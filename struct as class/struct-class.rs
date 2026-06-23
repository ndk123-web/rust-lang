struct User {
    name: String,
    age: u8,
}

impl User {
    fn get_name(&self)-> &String {
        &self.name
    }

    fn get_age(&self)-> &u8 {
        &self.age
    }
}

fn main() {
    let u: User = User {
        name: String::from("Ndk"),
        age: 18,
    };

    let age = u.get_age();
    let name = u.get_name();

    println!("Name: {}", name);
    println!("Age: {}", age);
}
