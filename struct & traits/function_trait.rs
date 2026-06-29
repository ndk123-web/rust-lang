/**
 * These file have 2 types of examples
 * 1. make sound with normal function
 * 2. make sound with generic functions
 */

struct Dog;

trait Speak {
    fn speak(&self);
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Boww");
    }
}

// Accept any type that implements the Speak trait.
fn make_sound(animal_speak: &impl Speak) {
    animal_speak.speak();
}

// Accept any type that implements the Speak trait.
fn make_sound_generics<T>(animal: &T) where T: Speak {
    animal.speak();
}

fn main() {
    let d = Dog;

    make_sound(&d);
    make_sound_generics(&d);
}