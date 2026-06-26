/**
 * Ascii Code is subset of Unicode (UTF-8)
 * Rust inside handles all in Unicode
 * 
 * s.chars() it retursn unicode scalar values ("Hé😊" => H->1byte, é->2byte, 😊->3)
 */

fn main() {
    let s: String = String::from("Ndk");

    // error
    // println!("{}", s[0]);

    // getting the ascii value (78)
    println!("{:?}", s.as_bytes()[0]);

    // print chars 
    for char in s.chars() {
        print!("{}", char);
    }

    println!();

    let mut s = String::from("नमस्ते");
    // print non ascii values
    for char in s.chars() {
        print!("{}", char);
    }

    // push char
    s.push('!');

    // push str
    s.push_str("Bro");

    // concatenated
    let s1 = String::from("Hello");
    let s2 = String::from("Hello");

    // inside + overload string (left one moved, right one get the borrow)
    let s3 = s1 + &s2;
    // s1 is invalid now

    // concatenated without removing both then (neither s1, s2 not removed)
    let s1 = String::from("Hello");
    let s3 = format!("{} {}", s1, s2);

    // replaced
    let s = String::from("Hello Rust");
    let new_s = s.replace("Rust", "World");

    // split
    let s = String::from("apple,banana,orange");
    for fruit in s.split(",") {
        println!("{}", fruit);
    }

    // trim
    let s = String::from("         Ndk        ");
    println!("{}", s.trim());

    // contains 
    let s = String::from("apple,banana,orange");
    println!("banana contains: {}", s.contains("apple"));

    // 
}
