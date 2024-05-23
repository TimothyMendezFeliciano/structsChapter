struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

pub fn rectangles() {
    let rect1: Rectangle = Rectangle {
        width: 50,
        height: 30,
    };

    let methodRect: Rectangle = Rectangle {
        width: 87,
        height: 132,
    };

    println!("The area of the rectangle is {} square pixels", area(&rect1)); // Let the function borrow the struct, so that the main function retains ownership.

    println!("The area of the rectangle with method syntax is {}", methodRect.area());
}

// This provides a more descriptive understanding of the code
fn area(rectangle: &Rectangle) -> usize {
    rectangle.width * rectangle.height
}