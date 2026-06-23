/**
 * Primitive Types
     1. Signed
        i8,i16,i32,i64,i128
     2. Unsigned 
        u8,u16,u32,u64,u128
    3. bool
        bool
    4.
 */

fn main() {

    // array
    let arr = [1,2,4];
    let i= 1;
    println!("{}", arr[i]); 
    println!("{}", arr.len());

    // vector
    let v = vec![1,2,3];
    let lenv = v.len();
    println!("{}", lenv);

    // tuple
    let tup = ("Ndk", 1,2);
    println!("{}-{}-{}", tup.0, tup.1,tup.2);

    let (name, roll, grade) = tup;
    println!("{}-{}-{}", name, roll, grade);

}