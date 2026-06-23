
fn main() {

    // @type-1: while (true)
    let mut i = 0;
    loop {
        if i == 11 {
            break;
        }

        println!("{}", i);
        i = i + 1;
    }

    // @type-2: while
    let mut j = 0;

    while j < 5 {
        println!("{}", j);
        j += 1;
    }

    // @type-3: for (true)
    for i in 0..5 {
        println!("{}", i);
    }
    for i in 0..=5 {
        println!("{}", i);
    }

}