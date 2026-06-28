use crate::storage::global_todos::TodoApp;
use crate::utils::todo::{self, Todo};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::mem;

pub fn entire_save(data: &mut TodoApp) {
    if fs::read_to_string("data/todo.txt").unwrap().is_empty() {
        println!("FIRST SAVE");
        let mut formatted_data: String = String::new();

        for todo in data.todos.iter() {
            formatted_data.push_str(&format!(
                "{} | {} | {}\n",
                todo.id, todo.title, todo.completed
            ));
        }

        fs::write("data/todo.txt", formatted_data).unwrap();
        data.old_todos = mem::take(&mut data.todos);
        data.todos = mem::replace(&mut data.todos, Vec::new());
    } else {
        println!("Append");
        append_todos(&mut data.todos);
    }
}

pub fn append_todos(todos: &mut Vec<Todo>) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data/todo.txt")
        .unwrap();

    for data in todos.iter() {
        writeln!(file, "{} | {} | {}", data.id, data.title, data.completed).unwrap();
    }

    todos.clear();
}
