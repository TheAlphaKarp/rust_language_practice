fn main() {
    println!("5 * 5 = {}", multiply(5, 5));
}

fn multiply(number_1: i32, number_2: i32) -> i32 {
    // expression -> return i32
    return number_1 * number_2
}

fn add(number_1: i32, number_2: i32) -> i32 {
    // statement -> no return
    number_1 + number_2;
}

fn divide(number_1: i32, number_2: i32) -> i32 {
    // expression -> return i32
    number_1 / number_2
}
