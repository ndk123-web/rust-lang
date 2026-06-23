/**
 * It has to be utils.rs and utils/
 * modern-import/
 *      utils/
 *          math.rs
 *      utils.rs
 *      math.rs
 */
mod utils;

use utils::math::add;
use utils::math::sub;

fn main() {
    let num1: u8 = 100;
    let num2: u8 = 120;

    let add_res = add(num1, num2);
    let sub_res = sub(num1, num2);

    println!("Add->{}", add_res);
    println!("Sub->{}", sub_res);
}
