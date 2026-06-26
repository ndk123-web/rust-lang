/**
 * get always return Option<&T> , so we need to derefer that using *
 */
use std::collections::HashMap;

fn main() {
    let mut mp = HashMap::new();

    // insert
    mp.insert("key1", "value1");

    // get
    let value = mp.get("key1");
    match value {
        Some(value) => {
            println!("Value: {}", *value);
        }

        None => {
            println!("No Key Exists");
        }
    }

    // get short
    if let Some(value) = mp.get("key1") {  
        println!("Value: {}", *value);
    }

    // update
    mp.insert("key1", "ndk");

    // contains
    let is_there: bool = mp.contains_key("key");

    // iteration
    for (key, value) in &mp {
        println!("{}-{}", key, value);
    }

    // removex
    mp.remove("age");
}
