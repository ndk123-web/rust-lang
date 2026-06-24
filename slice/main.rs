/**
 * 4 types
 * 1. &static str => let a = "ndk";  // immutable
 * 2. string slice => s = String and slice = &s[..4]
 * 3. String
 * 4. mut String
 */

fn main() {
    // stack (type= &static str) (points to binary file)
    let s1 = "Ndk";

    // heap (type= String)
    let mut s2: String = String::from("Hello World");
    s2.replace_range(0..5, "NdkBr");

    // string slice is &str (reference to String)
    let s3 = &s2[0..5];

    // both are same (rust infers internally)
    // let s4 = "NNdk";
    let s4: &'static str = "NNdk";

    println!("{}-{}", s1, s2);
    println!("{}", s3);
    println!("{}", s4);
}
