use std::fmt;

#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

pub struct TodoApp {
    todos: Vec<Option<Todo>>,
}

impl TodoApp {
    pub fn addTodo(&mut self, value: Option<Todo>) {
        self.todos.push(value);
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Todo {{ id: {}, title: {}, completed: {} }}",
            self.id, self.title, self.completed
        )
    }
}
