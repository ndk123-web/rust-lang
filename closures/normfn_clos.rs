/**
 * Normal Function -> cant capture other scopes
 * Closure Function -> can capture other scopes
 */

fn main() {
    let outer_scope_var: i32 = 10;

    // closure borrowed x reference
    let add = |a, b: i32| -> i32 {

        // The compiler captures `outer_scope_var` inside the closure.
        // If it's captured by reference, the compiler automatically
        // dereferences it when you use `outer_scope_var` in the closure body.
        
        // a + b + *outer_scope_var
        a + b + outer_scope_var
    };

    println!("{}", add(10, 20));
    println!("{}", add(10, 20));
}
