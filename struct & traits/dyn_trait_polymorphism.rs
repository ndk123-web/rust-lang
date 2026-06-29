/**
 * Polymorphsism (Runtime) (One interface, many forms -> (Speak)-> Cat, Dog, ...)
 * what we can do is use `Box<dyn Speak>` (Box always has 8 byte in 64 bits system)
 * `dyn speak` meaning 
    - A value whose concrete type
        I don't know at compile time,
        but I guarantee
        it implements Speak.

 * We can't use like Vec = [Dog, Cat..] because rust want only one type 
 */

use std::fmt::Debug;

trait Speak {
    fn speak(&self);
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) {
        println!("Bark");
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("Meoww");
    }
}

fn main() {

    //@type-1
    // here `dyn Speak` means dynamic dispatch
    // in run time we will be know the Object but it implementing Speak thats for sure
    let mut v: Vec<Box<dyn Speak>> = Vec::new();

    let d = Dog;
    v.push(Box::new(d));

    let c = Cat;
    v.push(Box::new(c));

    for i in v.iter() {
        i.speak();
    }

    // @type-2
    // x is a trait object.
    // Concrete type (Dog) is hidden behind the Speak interface.
    // Method calls are resolved through the vtable (dynamic dispatch).
    let x: &dyn Speak = &Dog;
    x.speak();
}