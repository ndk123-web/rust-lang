/**
 * ================================
 * Rust Closures
 * ================================
 *
 * Closure = Anonymous Function
 *
 * Syntax:
 *      |parameters| -> ReturnType {
 *          body
 *      }
 *
 * Compiler automatically decides how to capture variables.
 *
 * Capture Priority (Least Restrictive)
 * -----------------------------------
 * 1. Immutable Borrow (&T)
 *      - Only reading the value.
 *
 * 2. Mutable Borrow (&mut T)
 *      - Modifying the value.
 *
 * 3. Move / Ownership (T)
 *      - Consuming the value.
 *      - drop()
 *      - Returning ownership
 *      - move keyword
 */

fn main() {

    println!("==============================");
    println!("1. Basic Closures");
    println!("==============================");

    let add = |a: u8, b: u8| -> u8 { a + b };

    // Compiler infers parameter types.
    let sub = |a, b| a - b;

    println!("{}", add(10, 20));
    println!("{}", sub(20, 10));



    println!("\n==============================");
    println!("2. Immutable Borrow Capture");
    println!("==============================");

    let s = String::from("Rust");

    // Closure only READS s.
    // Therefore compiler captures &String.
    let print = || {
        println!("{}", s);
    };

    print();

    // Still valid because ownership never moved.
    println!("{}", s);



    println!("\n==============================");
    println!("3. Mutable Borrow Capture");
    println!("==============================");

    let mut name = String::from("Ndk");

    // Closure modifies the String.
    // Therefore compiler captures &mut String.
    let mut change = || {
        name.push_str(" Patil");
    };

    change();

    println!("{}", name);



    println!("\n==============================");
    println!("4. Move Capture (move keyword)");
    println!("==============================");

    let city = String::from("Pune");

    // move forces ownership into closure.
    let show = move || {
        println!("{}", city);
    };

    show();

    // ERROR
    // city no longer belongs to main().
    //
    // println!("{}", city);



    println!("\n==============================");
    println!("5. Automatic Move");
    println!("==============================");

    let language = String::from("Rust");

    // drop() consumes ownership.
    // Even without move keyword,
    // compiler captures language by value.
    let consume = || {
        println!("{}", language);

        // Takes ownership.
        drop(language);
    };

    consume();

    // ERROR
    // language already moved.
    //
    // println!("{}", language);



    println!("\n==============================");
    println!("6. move does NOT execute");
    println!("==============================");

    let framework = String::from("Rocket");

    // Ownership moves immediately.
    let closure = move || {
        println!("{}", framework);
    };

    // Even before calling closure,
    // framework is already moved.

    // ERROR
    // println!("{}", framework);

    closure();



    println!("\n==============================");
    println!("7. Copy Types");
    println!("==============================");

    let x = 100;

    // i32 implements Copy.
    // move copies x instead of transferring ownership.
    let print_x = move || {
        println!("{}", x);
    };

    print_x();

    // Still valid.
    println!("{}", x);



    println!("\n==============================");
    println!("8. Capture Rules Summary");
    println!("==============================");

    /*
        Read only
            ↓
            &T

        Modify
            ↓
            &mut T

        Consume
            ↓
            T

        move keyword
            ↓
            T (forced)
    */
}