/*
 * let keyword variables are defaultly immutable here
 * let mut keyword so that afterward we can reassign that as well
 * shadowing each time it creates new data
 * const is like normal const (must need type)
 * variables are scope based
*/

fn main() {
    // compiler infers as let x:i32 = 200

    // @type-1 (let)
    let x = 200;
    println!("{}", x);
    // x = 201;
    // println!("{}", x);

    // @type-2 (let mut)
    let mut y = 10;
    println!("{}", y);

    y = 200;
    println!("{}", y);

    // @type-3 (shadowing)
    let a = 10;
    let a = a + 20;
    let a = a + 10;
    println!("{}", a);

    // @type-4 (constant)
    const PI: f64 = 3.1432;
    println!("{}", PI);

    // @type-5 (scope) variable is scope based
    let x = 20;
    {
        let x = 22;
        println!("{}", x);
    }
    println!("{}", x);
}
