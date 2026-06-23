pub fn add(num1: u8, num2: u8) -> u8 {
    num1 + num2
}

pub fn sub(num1: u8, num2: u8) -> u8 {
    if num1 > num2 {
        num1 - num2
    } else {
        num2 - num1
    }
}
