/**
 * `?` simply means , if Err then return Err(e) directly to caller; else return
 *
 * see internally
 *      - let text = read_helper(file_name)?
 *      -> match text {
 *              Ok(data) => data
 *              Err(err) => return Err(err);
 *          }
 *
 *      here if all right then return data , else return Err(err)
 */
use std::fs;

fn read_helper(file_name: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_name)
}

fn read_string(file_name: &str) -> Result<String, std::io::Error> {
    let text = read_helper(file_name)?;
    Ok(text)
}

fn main() {
    let text = read_string("todo.txt");

    match text {
        Ok(data) => {
            println!("Data: {}", data);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    println!("Yo Still running");
}
