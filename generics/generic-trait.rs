/**
 * Trait means here `PartialOrd` meanign T must support comaprables
 */

// here PartialOrd means T must supports (comaparable <,>,>=,<=)
fn max_num<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// to add multiple traits just  use +
use std::fmt::Display;
fn display<T: PartialOrd + Display>(data: T) {
    println!("{}", data);
}

fn modern_display<T>(data: T) where T: PartialOrd + Display + Clone, {
    println!("{}",data);
}

fn main() {
    let a: u8 = 10;
    let b: u8 = 11;

    let res: u8 = max_num(a, b);
    println!("Max-{}", res);

    display("Ndk");
    modern_display("NdkBro");
}
