/**
 * Attributes are like `#[attribute]` here something means we are telling compiler to something instruction for below code/block
 * Syntax is Like: `#[attribute]` or `#[attribute(arguments)]`
 */

#[derive(Debug, Clone, Copy, PartialEq)]
struct Todo<'a> {
    id: usize,
    title: &'a str,
}

fn main() {

    // Feature of Debug
    let t: Todo = Todo {
        id: 1,
        title: "ndk",
    };
    println!("{:?}", t);

    // feature of Clone
    let t2 = t.clone();
    println!("{:?}", t2);
    println!("{:?}", t);

    // feature of Copy
    let t3 = t2;
    println!("{:?}", t2);
    println!("{:?}", t3);
}
