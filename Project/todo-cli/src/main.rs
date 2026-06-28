mod storage;
mod utils;

use std::env;
use std::io::{self, Write};
use storage::global_todos::TodoApp;
use storage::storage::{append_todos, entire_save};
use utils::todo::Todo;

fn main() {
    let mut todo_storage: TodoApp = TodoApp::new();
    let mut dynamic_id: usize = 1;
    let args : Vec<String> = env::args().collect();
    if (args.len() == 3) {
        println!("{:?}", args); 
        todo_storage.show_todos();
        return;
    }

    loop {
        println!("======================");
        println!("Todo APP");
        println!("======================");

        println!();

        println!("Enter a Choice");
        println!("1. Add Todo");
        println!("2. Show Todo");
        println!("3. Save Todos");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        input = input.trim().to_string();

        match input.as_str() {
            "1" => {
                println!("> Enter Title: ");

                // most important (sometimes Enter Title won't come on the screen (by flushing getting all the buffer data to on the screen))
                io::stdout().flush().unwrap();

                let mut title: String = String::new();
                io::stdin().read_line(&mut title).expect("Failed");
                title = title.trim().to_string();

                let todo: Todo = Todo::new(dynamic_id, title, false);
                todo_storage.add_todo(todo);

                dynamic_id += 1;

                println!("Succesfully Added!");
            }

            "2" => {
                todo_storage.show_todos();
            }

            "3" => {
                entire_save(&mut todo_storage);
            }

            "q" | "Q" => {
                break;
            }

            _ => {
                println!("Invalid Choice");
            }
        }
    }
}
