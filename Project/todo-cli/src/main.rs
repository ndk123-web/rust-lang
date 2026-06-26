mod utils;
use utils::todo::Todo;

fn main() {
    let mut todos: Vec<Option<Todo>> = Vec::new();

    let todo1: Option<Todo> = Some(Todo {
        id: (1),
        title: String::from("ndk"),
        completed: true,
    });

    let todo2: Option<Todo> = Some(Todo {
        id: (2),
        title: String::from("vsh"),
        completed: false,
    });

    todos.push(todo1);
    todos.push(todo2);

    // let todos: TodoApp = TodoApp::new();

    for i in todos.iter() {
        println!("{:#?}", i.as_ref().unwrap());
    }
}
