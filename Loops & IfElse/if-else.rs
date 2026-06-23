/**
 * No Brackets are needed
 */
fn main() {
    let x = 18;

    // @type-1 normal if
    if x >= 18 {
        println!("Adult");
    } else {
        println!("Not Adult");
    }

    // @type-2 if is
    let n = if x >= 18 {
        return 1;
    } else {
        return 0;
    };
    println!("{}", n);
}
