/**
 * Saw Issue Of `Dangling Reference` (meaning points to the freed memory)
 *
 * Actual Logic of Lifetime here
 * 1. In below u can see get_lifetime<'a> (its just a lable)
 *      - that 'a means just shortest live of borrow reference
 *      - compiler assume shortest one lifetime as 'a
 */

// Issue: Dangling Reference
// fn get_name() -> &str {
//     let s: String = String::from("Ndk");
//     return &s;
// }

// fn dangling_issue() {
//     get_name();
// }

fn get_lifetime<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let s1: String = String::from("Ndk1");
    let s2: String = String::from("Ndk2");

    let res: &str;

    let borrow1 = &s1;
    {
        let borrow2 = &s2;

        // in get_lifetime= ('a) means min life of (borrow1, borrow2)
        // so it will be borrow2
        // so the res lifetime = borrow2
        res = get_lifetime(borrow1, borrow2);
        println!("{}", res);
    }

    // println!("{}", res);
}
