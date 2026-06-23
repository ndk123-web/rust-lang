struct Dog;
struct Cat;

// like abstract class / interface 
trait Animal {
    fn speak(&self);
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Boww");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meoww");
    }
}

fn main() {

    let dog = Dog;
    dog.speak();

    let cat = Cat;
    cat.speak();
}