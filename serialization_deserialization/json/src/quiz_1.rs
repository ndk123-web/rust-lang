use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

pub fn serialize_todo(todo: &Todo) -> String {
    let json_string = serde_json::to_string_pretty(&todo);

    let data = json_string.expect("Panic: Issue in Serialization");
    println!("{:?}", data);

    return data;
}

pub fn deserialize_todo() {
    let todo: Todo = Todo {
        id: 1,
        title: String::from("Title-1"),
        completed: false,
    };
    let json_string: String = serialize_todo(&todo);

    let d_todo: Todo = serde_json::from_str(&json_string).expect("Panic: Issue in Deserialization");
    println!("{:?}", d_todo);
}
