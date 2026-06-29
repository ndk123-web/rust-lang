/**
 * Concept: 
 *  - self -> actual value
 *  - Self -> means currently implement trait on struct , the struct (not object)
 */

const PI: f32 = 3.14;

use std::fmt::Debug;

trait Area {
    fn area(&self);

    #[allow(dead_code)]
    fn print_area(&self) where Self: Debug {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct Rectangle {
    h: u32,
    w: u32,
}

#[derive(Debug)]
struct Circle {
    r: u32,
}

impl Rectangle {
    fn new(h: u32, w: u32) -> Rectangle {
        Rectangle {h, w}
    }
}

impl Area for Rectangle {

    fn area(&self) {
        println!("{}", self.h*self.w);
    }
}  

impl Circle {
    fn new(r: u32) -> Circle {
        Circle {r}
    }
}

impl Area for Circle {
    fn area(&self) {
        println!("{}", PI * self.r as f32 * self.r as f32);
    }
}

fn main() {
    let r = Rectangle::new(10,10);
    r.print_area();
    r.area();

    let c = Circle::new(10);
    c.print_area();
    c.area();

    // new type (self means this `r` object)
    Area::area(&r);
}