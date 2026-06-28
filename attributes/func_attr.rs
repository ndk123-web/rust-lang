#[inline]
fn sqare(x: usize) -> usize {
    x * x
}

#[allow(dead_code, unused_variables, unused_mut, non_snake_case)]
fn no_fn_warning_used() {}

#[allow(dead_code)]
#[cfg(target_os = "windows")]
fn compile_only_os() {}

#[allow(dead_code)]
#[deprecated(since = "1.0", note = "Instead use new_function")]
fn old_function() {}

#[allow(dead_code)]
fn new_function() {}

fn main() {
    // inline optimize sqare function
    println!("{}", sqare(10));

    // deprecated feature
    old_function();
}
