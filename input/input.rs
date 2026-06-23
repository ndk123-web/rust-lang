use std::io;

fn main() {

    let mut name: String = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed");

    println!("{}", name);
}