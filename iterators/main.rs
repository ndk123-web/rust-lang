/**
 *
 * Iterator -> Object which gives one element at a time
 *
 * for x in nums -> moved nums
 * for x in nums.iter() => no move just give reference of each num (&x)
 * for x in nums.iter_mut() => no move just gives (&mut x)
 */

fn main() {
    let mut v = vec![10, 20, 30, 40];

    // 1. immutable iter
    // return &i32 of first element
    let mut iter = v.iter();

    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    // 2. mutable iter
    // return &mut i32
    for x in v.iter_mut() {
        *x = 1;
    }
    println!("{:?}", v);

    // 3. to destroy after
    // for x in v.into_iter() {
    //     *x = 1;
    // }
    // println!("{:?}", v);

    // chaining (like js)
     let v = vec![1, 2, 3, 4];

    let new_v: Vec<i32> = v
        .iter()
        .map(|x| *x * 2)
        .collect();

    println!("{:?}", new_v);
}
