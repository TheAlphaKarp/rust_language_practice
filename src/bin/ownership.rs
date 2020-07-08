fn main() {
    // function generates string and gives ownership to s1
    let s1 = gives_ownership();

    // s2 is now owner of String
    let s2 = String::from("hello");

    // Function takes ownership because of move and gives back to s3
    // This results in s2 being dropped.
    let s3 = takes_and_gives_back(s2);
} // s1 and s3 is dropped because of scope exit. s2 was already dropped because of move.


fn gives_ownership() -> String {

    let some_string = String::from("hello");

    some_string // semicolon would prevent return here.
    // also valid options:
    // return some_string;
    // return some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// revoking ownership
fn find_word(s: &String) -> &str {
    const BYTES: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // returning a slice
            return &s[..i];
        }
    }

    return &s[..];
}