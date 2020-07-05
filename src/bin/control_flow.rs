fn main() {
    let number = 3;

    if number < 3 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    for number in (1..4).rev() {

    }

    println!("The result is {}", result);
}