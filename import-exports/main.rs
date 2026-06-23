mod math;

fn main() {

    let a: u8 = 10;
    let b: u8 = 20;

    let res: u8 = math::add(a, b);

    println!("{}", res);
    println!("{}", math::PI);
}