pub fn rectangles() {
    let width1: usize = 30;
    let height1: usize = 50;

    println!("The area of the rectangle is {} square pixels", area(width1,height1));
}

fn area(width: usize, height: usize) -> usize {
    width * height
}