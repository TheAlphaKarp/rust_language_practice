fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;

    // invalid. s1 has been invalidated after copy to prevent double free error.
    println!("{} world!", s1);
}