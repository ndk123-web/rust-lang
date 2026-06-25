/**
 * Golden Rule: All data types has ownership, only excluding references (&s, &mut s)
 *
 * Generic Type-1 (identity)
 * Generic Type-2 (struct)
 * Generic Type-3 (multi generic)
 */

// @type-1 generic function
fn identity<T>(a: T) -> T {
    a
}

// @type-2 generic struct
struct Point<T> {
    x: T,
    y: T,
}

// @type-3 multiple generic struct
struct User<T, U, V> {
    age: T,
    name: U,
    rank: V,
}

fn main() {
    // @type-1
    let a: u8 = 120;
    println!("{}", identity(a));
    let a: f64 = 1.21;
    println!("{}", identity(a));

    // @type-2
    let a: u8 = 120;
    let p = Point { x: &a, y: &a };
    println!("{}-{}", p.x, p.y);

    // @type-3
    let name: &str = "Ndk";
    let age: u8 = 21;
    let rank: u16 = 2;

    let u = User { name, age, rank };
    println!("{}-{}-{}", u.name, u.age, u.rank);
}
