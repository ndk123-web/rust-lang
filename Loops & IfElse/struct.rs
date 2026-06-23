struct User {
    name: String,
    age: u8,
}

fn main() {
    let u = User {
        name: String::from("Ndk"),
        age: 21,
    };

    println!("{}-{}", u.name, u.age);
}
