struct Rectangle {
    width: usize,
    height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        (self.height * self.width) >= (rect2.height * rect2.width)
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

    println!("Can rect1 hold rect2 inside? {}", rect1.can_hold(&methodRect));
    println!("Can rect2 hold rect1 inside? {}", methodRect.can_hold(&rect1));
}

// This provides a more descriptive understanding of the code
fn area(rectangle: &Rectangle) -> usize {
    rectangle.width * rectangle.height
}