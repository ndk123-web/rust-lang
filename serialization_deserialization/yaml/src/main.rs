/**
 * Thats all about the YAML file handling in Rust.
 * About more features, same as the JSON file handling. (rename, rename_all, default, skip, skip_serializing, skip_serializing_if, skip_deserializing, skip_deserializing_if, etc.)
 */

use serde::{Serialize, Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String, 
    age: u8, 
    email: String, 
}

fn main() {
    let user = User {
        name: String::from("John Doe"),
        age: 30,
        email: String::from("john.doe@example.com"),
    };

    let serialized = serde_yaml::to_string(&user);
    println!("Serialized: {:?}", serialized);

    let deserialized: User = serde_yaml::from_str(&serialized.unwrap()).expect("Issue deserializing");
    println!("Deserialized: {:?}", deserialized); 
}