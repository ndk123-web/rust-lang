/**
 * Serialize => (Rust struct --> Json String)
 * Deserialize => (Json String --> Rust Struct)
 */
mod quiz_1;

use quiz_1::{deserialize_todo};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    role: String,
}

fn main() {
    // @type-1 Serialize
    let user: User = User {
        name: String::from("Ndk"),
        age: 21,
        role: String::from("121102"),
    };

    let json_string = serde_json::to_string_pretty(&user);
    println!("{:?}", json_string);

    // @type-2 Deserialize
    let again: User = serde_json::from_str(&json_string.unwrap()).expect("Issue");
    println!("{:?}", again);

    // Quiz-1
    deserialize_todo();
}
