/**
* works when data inside heap
* Every value has one owner.
* Only one owner at a time.
*
*
* in borrowing
* Either:
   - Many readers
   OR
   - One writer
   Never both together
*/

// just borrowed , not ownership
fn print_string(ss: &String) {
    println!("{}", ss);
}

fn main() {
    let s = String::from("ndk");

    // ownership of s is moved to s1 and now s is empty or not exists (works when data inside heap)
    let s1 = s;
    println!("{}", s1);

    // borrowing
    let mut ss = String::from("ndkBro");
    print_string(&ss);

    let r1 = &ss;
    let r2 = &ss;
    println!("{}-{}", r1, r2);

    let r3 = &mut ss;
    r3.push_str("Broo");
    println!("{}", r3);
}
