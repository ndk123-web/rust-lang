/**
 * We can write directly u know like (impl Dog, impl Cat)
 * we write because rust now know all `Dog` and `Cat` implement trait `Speak`
 */

struct Dog;
struct Cat;

trait Speak {
    fn speak(&self);
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Boww");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("meoww");
    }
}

fn main() {
    let d= Dog;
    d.speak();
}