use std::fmt;

#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: usize, title: String, completed: bool) -> Todo {
        Todo {
            id,
            title,
            completed,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn mark_incomplete(&mut self) {
        self.completed = false;
    }

    fn destroy(self) {
        println!("destroyedd id: {}", self.id);
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
