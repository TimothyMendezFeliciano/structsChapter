struct Rectangle {
    width: usize,
    height: usize,
}

pub fn rectangles() {
    let rect1: Rectangle = Rectangle {
        width: 50,
        height: 30,
    };

    println!("The area of the rectangle is {} square pixels", area(&rect1)); // Let the function borrow the struct, so that the main function retains ownership.
}

// This provides a more descriptive understanding of the code
fn area(rectangle: &Rectangle) -> usize {
    rectangle.width * rectangle.height
}