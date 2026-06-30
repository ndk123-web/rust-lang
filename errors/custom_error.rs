use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
enum MyError {
    FileNotFound,
    InvalidFileName,
}

#[allow(dead_code)]
fn read_helper(file_name: &str) -> Result<String, MyError> {
    match fs::read_to_string(file_name) {
        Ok(data) => Ok(data),
        Err(_) => Err(MyError::FileNotFound)
    }
}

#[allow(dead_code)]
fn read_string(file_name: &str) -> Result<String, MyError> {
    read_helper(file_name)
}

fn main() -> Result<(), MyError> {
    let data = read_string("todo.txt")?;
    
    println!("{}", data);
    Ok(())
}
