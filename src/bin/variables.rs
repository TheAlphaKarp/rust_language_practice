fn main() {
    let age = 32;
    let mut name = "Karp";
    const BIRTHDAY: &str = "1998";

    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("first element {}", tup.0);
    println!("Destructured {}", x);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    // forbidden
    println!("Element value: {}", a[index]);
}