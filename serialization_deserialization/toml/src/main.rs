/**
 * Thats all the Toml
 * For more its uses for rules `serde`
 * and for actual ser/deser it uses `toml`
 *
 * and have same features like json,yaml have (rename, rename_all, skil, ..etc)
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
    };

    let serialized = toml::to_string(&user);
    println!("Serialized: {:?}", serialized);

    let deserialize: Result<User, toml::de::Error> = toml::from_str(&serialized.unwrap());
    println!("Deserialize: {:?}", deserialize.unwrap());
}
