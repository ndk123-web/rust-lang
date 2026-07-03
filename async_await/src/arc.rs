/**
 * Arc only solve `many owners` not Mutations
 *
 * Problem:
 *  - when we working in Multiple threads:
 *  - Assume main thread has data1, worker thread has data2
 *  - see data1 and data2 are both owners pointing to same location
 *  - if we took &data1 to worker thread then if main exsits then in worker thread the data1 may be invalid
 *  - thats why we use Arc that provides multiple owners to the same data in heap
 */
use std::sync::Arc;

#[allow(dead_code)]
pub fn arc_concept() {
    let name = Arc::new(String::from("Ndk"));

    // just cloned the reference of name
    let a = Arc::clone(&name);
    let b = Arc::clone(&name);

    println!("{}", a);
    println!("{}", b);
}
