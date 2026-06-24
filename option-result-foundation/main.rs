/**
 * Concept of Enums Entierely
 */

fn main() {
    // Option has "Some" and "None"
    let age: Option<u8> = Some(10);

    match age {
        Some(value) => {
            println!("{}", value)
        }

        None => {
            println!("None")
        }
    }

    // Result has "Ok" and "Err"
    let name: Result<String, String> = Ok(String::from("Ndk"));

    match name {
        Ok(value) => {
            println!("{}", value);
        }

        Err(err) => {
            println!("{}", err);
        }
    }
}
