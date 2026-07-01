use serde::{Deserialize, Serialize};
use std::fmt::Debug;


/**
 * Now Serialize -> (Rust struct --> Json String) (name -> user_name, age -> user_age, role -> user_role)
 * Now Deserialize -> (Json String --> Rust struct) (user_name -> name, user_age -> age, user_role -> role)
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct User {

    #[serde(rename = "user_name")]
    name: String,

    #[serde(rename = "user_age")]
    age: u8,

    #[serde(rename = "user_role")]
    role: String,
}