/**
 * Why Vector ?
 * because :
 *      - dynamic size
 *      - supports len, push, pop, clear, contains, insert
 */

fn main() {
    // way 1: call static method of struct Vec
    let mut v1: Vec<u8> = Vec::new();
    v1.push(10);
    println!("{}", v1[0]);

    // way 2: call macro
    let mut v1: Vec<u8> = vec![1, 2, 3];
    println!("{}", v1[0]);

    // 1.push
    v1.push(4);

    // 2. pop return Option<T>
    let value: Option<u8> = v1.pop();
    match value {
        Some(value) => {
            println!("{}", value);
        }

        None => {
            println!("Value is None");
        }
    }

    println!("{}", value.unwrap());

    // remove (takes index)
    v1.remove(2);

    // 3. immutable iterating
    for num in &v1 {
        print!(" {}", num);
    }
    println!("Removed");

    // 4. mutable iterating
    for num in &mut v1 {
        *num = *num + 1;
        print!(" {}", *num);
    }

    // 5. loop + destroy vector
    // for num in v1 {
    //     println!("{}", num);
    // }

    // 6. insert
    v1.insert(1, 100);
    println!("{}", v1[1]);

    // 7. contains (takes reference)
    println!("100 contains: {}", v1.contains(&100));

    // formatters => ( {}, {:?}, {:#?} )
    println!("{:?}", v1);
    println!("{:#?}", v1);

    // get element 3 ways
    let a = v1[0];
    let b = v1.get(0);
    let c = v1.get_mut(0);
}
