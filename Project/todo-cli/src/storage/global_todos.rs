use crate::utils::todo::Todo;
use std::fmt;
use std::fs;
use std::mem;

// enable ({:?} and {:#?})
#[derive(Debug)]
pub struct TodoApp {
    pub todos: Vec<Todo>,
    pub old_todos: Vec<Todo>,
}

impl TodoApp {
    pub fn new() -> TodoApp {
        return TodoApp {
            todos: Vec::new(),
            old_todos: Vec::new(),
        };
    }

    pub fn add_todo(&mut self, data: Todo) {
        self.todos.push(data);
    }

    pub fn show_todos(&mut self) {
        self.old_todos.clear();

        
        let contents = fs::read_to_string("data/todo.txt").unwrap();

        for line in contents.lines() {
            println!("{:?}", line);

            let parts: Vec<&str> = line.split("|").collect();

            let id: usize = parts[0].trim().parse().unwrap();
            let title = parts[1].trim().to_string();
            let completed: bool = parts[2].trim().parse().unwrap();

            let todo: Todo = Todo::new(id, title, completed);
            self.old_todos.push(todo);
        }

        println!("New Todos: ");
        println!("{:#?}", self.todos);

        println!("Old Todos: ");
        println!("{:#?}", self.old_todos);
    }
}
