// structs can hold methods
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // object function
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectange {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(10);

    println!("The area of the rectangle is {} square pixels.", rectangle.area())
}